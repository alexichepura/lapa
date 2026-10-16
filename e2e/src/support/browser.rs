//! Thin wrapper around playwright-rs: launches Chromium with a fixed viewport
//! so screenshots are comparable across runs.

use std::path::Path;

use anyhow::Result;
use playwright_rs::{
    Browser, BrowserContext, BrowserContextOptions, LaunchOptions, Page, Playwright, Viewport,
};

pub struct BrowserSession {
    pub playwright: Playwright,
    pub browser: Browser,
    #[allow(dead_code)]
    pub context: BrowserContext,
    pub page: Page,
}

pub async fn launch() -> Result<BrowserSession> {
    let playwright = Playwright::launch().await?;
    // Extra Chromium flags (space-separated), ignored when unset.
    let args: Vec<String> = std::env::var("LAPA_BROWSER_ARGS")
        .ok()
        .map(|v| v.split_whitespace().map(str::to_string).collect())
        .unwrap_or_default();
    let mut options = LaunchOptions::default();
    if !args.is_empty() {
        options.args = Some(args);
    }
    // `LAPA_BROWSER_HEADLESS=0` runs a real window.
    if let Ok(v) = std::env::var("LAPA_BROWSER_HEADLESS")
        && matches!(v.trim(), "0" | "false" | "no")
    {
        options.headless = Some(false);
    }
    let browser = playwright.chromium().launch_with_options(options).await?;
    let context = browser
        .new_context_with_options(
            BrowserContextOptions::builder()
                .viewport(Viewport {
                    width: 1280,
                    height: 800,
                })
                .build(),
        )
        .await?;
    let page = context.new_page().await?;
    Ok(BrowserSession {
        playwright,
        browser,
        context,
        page,
    })
}

impl BrowserSession {
    pub async fn shutdown(self) -> Result<()> {
        let _ = self.browser.close().await;
        let _ = self.playwright.shutdown().await;
        Ok(())
    }
}

/// One-off screenshot of an already-running server, used for ad-hoc inspection.
pub async fn shot(url: &str, out: &Path) -> Result<()> {
    let session = launch().await?;
    session.page.goto(url, None).await?;
    session.page.wait_for_load_state(None).await?;
    let wait_ms: u64 = std::env::var("LAPA_SHOT_WAIT_MS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    if wait_ms > 0 {
        tokio::time::sleep(std::time::Duration::from_millis(wait_ms)).await;
    }
    if let Some(parent) = out.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    session.page.screenshot_to_file(out, None).await?;
    for message in session.page.console_messages() {
        println!("[console:{}] {}", message.type_(), message.text());
    }
    for error in session.page.page_errors() {
        println!("[pageerror] {}", error.lines().next().unwrap_or_default());
    }
    session.shutdown().await?;
    Ok(())
}
