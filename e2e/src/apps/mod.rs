//! Registry of the applications under test. Each frontend (`site` now, `admin`
//! later) is one module implementing [`AppUnderTest`] and contributing cases.

pub mod admin;
pub mod site;

use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;

use anyhow::Result;
use playwright_rs::Page;

use crate::support::server::ServerLog;
use crate::support::snapshot;

pub type CaseFuture<'a> = Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>>;

pub struct Case {
    pub name: &'static str,
    pub run: for<'a> fn(&'a CaseCtx) -> CaseFuture<'a>,
}

/// A frontend that can be booted and exercised.
pub trait AppUnderTest {
    fn name(&self) -> &'static str;
    /// Directory passed to `cargo leptos` (contains the Leptos manifest).
    fn server_dir(&self) -> PathBuf;
    fn cases(&self) -> Vec<Case>;
}

/// Everything a case needs: the live page, the base URL, artifact paths and the
/// captured server log.
pub struct CaseCtx {
    pub app: &'static str,
    pub base_url: String,
    pub page: Page,
    pub artifacts_dir: PathBuf,
    #[allow(dead_code)]
    pub server_log: ServerLog,
    pub update: bool,
}

impl CaseCtx {
    pub fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    pub async fn goto(&self, path: &str) -> Result<()> {
        self.page.goto(&self.url(path), None).await?;
        self.page.wait_for_load_state(None).await?;
        // Let hydration settle (or fail) before asserting/screenshotting so the
        // captured DOM is stable.
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        Ok(())
    }

    /// Like [`goto`](Self::goto) but also returns the HTTP status (e.g. to
    /// assert a 404 page).
    pub async fn goto_status(&self, path: &str) -> Result<u16> {
        let response = self.page.goto(&self.url(path), None).await?;
        self.page.wait_for_load_state(None).await?;
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        Ok(response.map(|r| r.status()).unwrap_or(0))
    }

    /// Poll until the first element matching `selector` is a loaded `<img>`
    /// (the CDN resizes on first request, so it can take a moment).
    pub async fn wait_for_image(&self, selector: &str, secs: u64) -> Result<()> {
        let expr = format!(
            "() => {{ const i = document.querySelector({selector:?}); \
             return !!i && i.complete && i.naturalWidth > 0; }}"
        );
        for _ in 0..secs * 4 {
            let ok: bool = self
                .page
                .evaluate(&expr, None::<&()>)
                .await
                .unwrap_or(false);
            if ok {
                return Ok(());
            }
            tokio::time::sleep(std::time::Duration::from_millis(250)).await;
        }
        anyhow::bail!("image {selector:?} never loaded within {secs}s")
    }

    pub async fn snapshot(&self, name: &str) -> Result<()> {
        snapshot::compare(&self.page, self.app, name, &self.artifacts_dir, self.update).await
    }

    pub fn console_errors(&self) -> Vec<String> {
        self.page
            .console_messages()
            .into_iter()
            .filter(|m| m.type_() == "error")
            .map(|m| {
                let loc = m.location();
                let text = m.text();
                let first = text.lines().next().unwrap_or_default();
                let brief: String = first.chars().take(200).collect();
                format!("{brief} [{}:{}]", loc.url, loc.line_number)
            })
            .collect()
    }

    pub fn clear_browser_errors(&self) {
        self.page.clear_console_messages();
        self.page.clear_page_errors();
    }

    /// Browser-noise that is not actionable for these smoke tests.
    pub fn is_benign(message: &str) -> bool {
        message.contains("/favicon.ico") || message.contains(".map")
    }

    pub fn page_errors(&self) -> Vec<String> {
        self.page.page_errors()
    }
}
