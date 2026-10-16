//! Visual regression: compare a page against a committed baseline, or rewrite
//! the baseline when running in update mode.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use playwright_rs::{Animations, Page, ScreenshotAssertionOptions, expect_page};

pub fn snapshots_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("snapshots")
}

/// When set, `ctx.snapshot` becomes a no-op. Lets CI (or another OS without
/// committed baselines) exercise the behavioural cases without pixel diffs.
pub fn skipped() -> bool {
    matches!(
        std::env::var("LAPA_E2E_SKIP_SNAPSHOTS").as_deref(),
        Ok(v) if !matches!(v.trim(), "" | "0" | "false" | "no")
    )
}

pub async fn compare(
    page: &Page,
    app: &str,
    name: &str,
    artifacts_dir: &Path,
    update: bool,
) -> Result<()> {
    let baseline_dir = snapshots_dir().join(app).join(std::env::consts::OS);
    tokio::fs::create_dir_all(&baseline_dir)
        .await
        .with_context(|| format!("creating baseline dir {}", baseline_dir.display()))?;
    let baseline = baseline_dir.join(format!("{name}.png"));

    let options = ScreenshotAssertionOptions::builder()
        .animations(Animations::Disabled)
        .max_diff_pixel_ratio(0.01)
        .threshold(0.2)
        .update_snapshots(update)
        .build();

    if let Err(err) = expect_page(page)
        .to_have_screenshot(&baseline, Some(options))
        .await
    {
        tokio::fs::create_dir_all(artifacts_dir).await.ok();
        let actual = artifacts_dir.join(format!("{name}.actual.png"));
        let _ = page.screenshot_to_file(&actual, None).await;
        return Err(anyhow::anyhow!(
            "screenshot `{name}` mismatch: {err} (actual saved to {})",
            actual.display()
        ));
    }
    Ok(())
}
