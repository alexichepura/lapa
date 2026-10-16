//! The Lapa admin UI (Leptos SSR + session auth) under test.

use std::path::{Path, PathBuf};
use std::time::Duration;

use playwright_rs::expect;

use crate::apps::{AppUnderTest, Case, CaseCtx, CaseFuture};
use crate::support::seed;

pub struct AdminApp;

impl AppUnderTest for AdminApp {
    fn name(&self) -> &'static str {
        "admin"
    }

    fn server_dir(&self) -> PathBuf {
        // `admin` is a workspace member; cargo-leptos is invoked from the root.
        Path::new(env!("CARGO_MANIFEST_DIR")).join("..")
    }

    fn cases(&self) -> Vec<Case> {
        vec![
            Case {
                name: "login-page-render",
                run: case_login_page,
            },
            Case {
                name: "login-flow",
                run: case_login_flow,
            },
            Case {
                name: "pages-render",
                run: case_pages_render,
            },
        ]
    }
}

/// Unauthenticated `/` must land on the login form (this case runs first, in a
/// fresh browser context).
fn case_login_page<'a>(ctx: &'a CaseCtx) -> CaseFuture<'a> {
    Box::pin(async move {
        ctx.goto("/").await?;
        anyhow::ensure!(
            !dashboard_visible(ctx).await?,
            "expected to be logged out, but the dashboard rendered"
        );
        expect(ctx.page.locator("text=Log in"))
            .to_be_visible()
            .await?;
        expect(ctx.page.locator("input[name=username]"))
            .to_be_visible()
            .await?;
        expect(ctx.page.locator("input[name=password]"))
            .to_be_visible()
            .await?;
        ctx.snapshot("login").await?;
        Ok(())
    })
}

/// The seeded user can log in and reach the dashboard.
fn case_login_flow<'a>(ctx: &'a CaseCtx) -> CaseFuture<'a> {
    Box::pin(async move {
        login(ctx).await?;
        expect(ctx.page.locator("h1"))
            .to_contain_text("Dashboard")
            .await?;
        expect(ctx.page.locator("text=e2e-admin"))
            .to_be_visible()
            .await?;
        // The router's progress bar lingers briefly after navigation and the
        // stats `<Transition>` resolves asynchronously; wait for both so the
        // screenshot is deterministic.
        wait_until_hidden(ctx, ".RoutingProgress", 10).await?;
        expect(ctx.page.locator("text=All time"))
            .to_be_visible()
            .await?;
        ctx.snapshot("dashboard").await?;
        Ok(())
    })
}

/// Authenticated navigation across the admin sections stays error-free.
fn case_pages_render<'a>(ctx: &'a CaseCtx) -> CaseFuture<'a> {
    Box::pin(async move {
        login(ctx).await?;
        let mut problems = Vec::new();
        for path in ["/", "/product", "/post-category", "/post", "/settings"] {
            ctx.clear_browser_errors();
            ctx.goto(path).await?;
            let mut page_problems: Vec<String> = ctx
                .console_errors()
                .into_iter()
                .filter(|m| !CaseCtx::is_benign(m))
                .collect();
            page_problems.extend(
                ctx.page_errors()
                    .into_iter()
                    .map(|e| e.lines().next().unwrap_or_default().to_string()),
            );
            if !page_problems.is_empty() {
                problems.push(format!("{path}:\n    {}", page_problems.join("\n    ")));
            }
        }
        if !problems.is_empty() {
            anyhow::bail!("browser errors:\n  {}", problems.join("\n  "));
        }
        Ok(())
    })
}

/// Log in through the form, or no-op when the session is already established
/// (cases share one browser context, so the cookie survives across cases).
async fn login(ctx: &CaseCtx) -> anyhow::Result<()> {
    ctx.goto("/").await?;
    if dashboard_visible(ctx).await? {
        return Ok(());
    }

    ctx.page
        .locator("input[name=username]")
        .fill(seed::ADMIN_USERNAME, None)
        .await?;
    ctx.page
        .locator("input[name=password]")
        .fill(seed::ADMIN_PASSWORD, None)
        .await?;
    ctx.page.locator("input[type=submit]").click(None).await?;

    for _ in 0..80 {
        if dashboard_visible(ctx).await? {
            return Ok(());
        }
        tokio::time::sleep(Duration::from_millis(250)).await;
    }
    anyhow::bail!(
        "never reached the dashboard after login (url={})",
        ctx.page.url()
    )
}

/// The dashboard's own `<h1>` (the nav also has a "Dashboard" link).
async fn dashboard_visible(ctx: &CaseCtx) -> anyhow::Result<bool> {
    let h1 = ctx.page.locator("h1");
    // `count()` is immediate; `inner_text()` would wait out the timeout when the
    // element is absent (e.g. on the login page).
    if h1.count().await? == 0 {
        return Ok(false);
    }
    Ok(h1.inner_text().await?.trim() == "Dashboard")
}

/// Poll until `selector` is no longer visible (best effort after `secs`).
async fn wait_until_hidden(ctx: &CaseCtx, selector: &str, secs: u64) -> anyhow::Result<()> {
    for _ in 0..secs * 4 {
        if !ctx
            .page
            .locator(selector)
            .is_visible()
            .await
            .unwrap_or(false)
        {
            return Ok(());
        }
        tokio::time::sleep(Duration::from_millis(250)).await;
    }
    Ok(())
}
