//! Registry of the applications under test. Each frontend (`site`, `admin`) is
//! one module implementing [`AppUnderTest`] and contributing cases; a suite may
//! span several apps (see `integration`).

pub mod admin;
pub mod integration;
pub mod site;

use std::collections::HashMap;
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

use anyhow::Result;
use playwright_rs::Page;

use crate::support::snapshot;

pub type CaseFuture<'a> = Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>>;

pub struct Case {
    pub name: &'static str,
    pub run: for<'a> fn(&'a CaseCtx) -> CaseFuture<'a>,
}

/// A frontend that can be booted and exercised, or a suite that spans several.
pub trait AppUnderTest {
    fn name(&self) -> &'static str;
    /// Directory passed to `cargo leptos` (contains the Leptos manifest).
    fn server_dir(&self) -> PathBuf;
    fn cases(&self) -> Vec<Case>;
}

/// Everything a case needs: the live page, the booted apps' base URLs, artifact
/// paths and the update flag.
pub struct CaseCtx {
    /// Suite name; also the snapshot directory.
    pub app: &'static str,
    /// Server used by the app-less helpers (`goto`, `url`, `snapshot`-adjacent).
    pub primary: &'static str,
    /// Base URL per booted server, keyed by app name.
    pub base_urls: HashMap<&'static str, String>,
    pub page: Page,
    pub artifacts_dir: PathBuf,
    /// Postgres URL, for cases that need to arrange DB state directly.
    pub database_url: String,
    pub update: bool,
    /// Server log lines (substrings) the suite expects, e.g. an intentional
    /// 500. The harness ignores matching fatal lines.
    pub allowed_server_errors: Arc<Mutex<Vec<String>>>,
}

impl CaseCtx {
    /// Declare a server-log line (substring) this case expects to produce. The
    /// harness filters matching fatal lines instead of failing the suite.
    pub fn allow_server_error(&self, needle: &str) {
        self.allowed_server_errors
            .lock()
            .unwrap()
            .push(needle.to_string());
    }

    pub fn url_for(&self, app: &str, path: &str) -> String {
        let base = self
            .base_urls
            .get(app)
            .unwrap_or_else(|| panic!("no server booted for app `{app}`"));
        format!("{base}{path}")
    }

    pub async fn goto(&self, path: &str) -> Result<()> {
        self.goto_for(self.primary, path).await
    }

    /// Navigate on a specific app (for cross-app suites).
    pub async fn goto_for(&self, app: &str, path: &str) -> Result<()> {
        self.page.goto(&self.url_for(app, path), None).await?;
        self.page.wait_for_load_state(None).await?;
        // Let hydration settle (or fail) before asserting/screenshotting so the
        // captured DOM is stable.
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        Ok(())
    }

    /// Like [`goto`](Self::goto) but also returns the HTTP status (e.g. to
    /// assert a 404 page).
    pub async fn goto_status(&self, path: &str) -> Result<u16> {
        self.goto_status_for(self.primary, path).await
    }

    pub async fn goto_status_for(&self, app: &str, path: &str) -> Result<u16> {
        let response = self.page.goto(&self.url_for(app, path), None).await?;
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
        if snapshot::skipped() {
            return Ok(());
        }
        snapshot::compare(&self.page, self.app, name, &self.artifacts_dir, self.update).await
    }

    /// Run SQL against the ephemeral database (state arrangement for a case).
    pub async fn exec_sql(&self, sql: &str) -> Result<()> {
        let (client, connection) =
            tokio_postgres::connect(&self.database_url, tokio_postgres::NoTls).await?;
        tokio::spawn(async move {
            let _ = connection.await;
        });
        client.batch_execute(sql).await?;
        Ok(())
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
