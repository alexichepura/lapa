//! Cross-app cases: a write through the admin UI is then observed on the public
//! site. The suite boots both servers against one database (see `main.rs`).

use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::apps::{AppUnderTest, Case, CaseCtx, CaseFuture, admin};

pub struct IntegrationApp;

impl AppUnderTest for IntegrationApp {
    fn name(&self) -> &'static str {
        "integration"
    }

    fn server_dir(&self) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("..")
    }

    fn cases(&self) -> Vec<Case> {
        vec![Case {
            name: "settings-roundtrip",
            run: case_settings_roundtrip,
        }]
    }
}

/// Update the site settings in the admin UI, then read them back from the
/// public site (which loads settings from the database on every request).
fn case_settings_roundtrip<'a>(ctx: &'a CaseCtx) -> CaseFuture<'a> {
    Box::pin(async move {
        admin::login(ctx).await?;

        let home_text = "Roundtrip home text (e2e)";
        let robots = "User-agent: e2e-roundtrip";

        ctx.goto_for("admin", "/settings").await?;
        ctx.page
            .locator("textarea[name=home_text]")
            .fill(home_text, None)
            .await?;
        ctx.page
            .locator("input[type=submit][value=\"Update home data\"]")
            .click(None)
            .await?;
        ctx.page
            .locator("textarea[name=robots_txt]")
            .fill(robots, None)
            .await?;
        ctx.page
            .locator("input[name=site_url]")
            .fill("https://roundtrip.example", None)
            .await?;
        ctx.page
            .locator("input[type=submit][value=\"Update site data\"]")
            .click(None)
            .await?;

        wait_for_site(ctx, "/robots.txt", robots, 20).await?;
        wait_for_site(ctx, "/", home_text, 20).await?;
        Ok(())
    })
}

/// Poll the public site until `path` contains `needle`.
async fn wait_for_site(ctx: &CaseCtx, path: &str, needle: &str, secs: u64) -> anyhow::Result<()> {
    for _ in 0..secs {
        ctx.goto_for("site", path).await?;
        if ctx.page.content().await?.contains(needle) {
            return Ok(());
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    anyhow::bail!("site {path} never contained {needle:?}")
}
