//! The Lapa public site (Leptos SSR) under test.

use std::path::{Path, PathBuf};

use playwright_rs::expect;

use crate::apps::{AppUnderTest, Case, CaseCtx, CaseFuture};
use crate::support::seed;

pub struct SiteApp;

impl AppUnderTest for SiteApp {
    fn name(&self) -> &'static str {
        "site"
    }

    fn server_dir(&self) -> PathBuf {
        // The root crate is the `site` Leptos app.
        Path::new(env!("CARGO_MANIFEST_DIR")).join("..")
    }

    fn cases(&self) -> Vec<Case> {
        vec![
            Case {
                name: "home-render",
                run: case_home,
            },
            Case {
                name: "product-page-render",
                run: case_product_page,
            },
            Case {
                name: "post-page-render",
                run: case_post_page,
            },
            Case {
                name: "console-clean",
                run: case_console_clean,
            },
        ]
    }
}

fn case_home<'a>(ctx: &'a CaseCtx) -> CaseFuture<'a> {
    Box::pin(async move {
        ctx.goto("/").await?;
        expect(ctx.page.locator("text=Welcome to Lapa"))
            .to_be_visible()
            .await?;
        // The seeded home text and both list sections render server-side.
        expect(ctx.page.locator(format!("text={}", seed::HOME_TEXT)))
            .to_be_visible()
            .await?;
        expect(ctx.page.locator("text=E2E Product H1"))
            .to_be_visible()
            .await?;
        expect(ctx.page.locator("text=E2E Post H1"))
            .to_be_visible()
            .await?;
        ctx.snapshot("home").await?;
        Ok(())
    })
}

fn case_product_page<'a>(ctx: &'a CaseCtx) -> CaseFuture<'a> {
    Box::pin(async move {
        ctx.goto(&format!("/product/{}", seed::PRODUCT_SLUG))
            .await?;
        expect(ctx.page.locator("h1"))
            .to_contain_text("E2E Product H1")
            .await?;
        expect(ctx.page.locator("text=E2E product body."))
            .to_be_visible()
            .await?;
        ctx.snapshot("product").await?;
        Ok(())
    })
}

fn case_post_page<'a>(ctx: &'a CaseCtx) -> CaseFuture<'a> {
    Box::pin(async move {
        ctx.goto(&format!(
            "/{}/{}",
            seed::POST_CATEGORY_SLUG,
            seed::POST_SLUG
        ))
        .await?;
        expect(ctx.page.locator("h1"))
            .to_contain_text("E2E Post H1")
            .await?;
        expect(ctx.page.locator("text=E2E post body."))
            .to_be_visible()
            .await?;
        ctx.snapshot("post").await?;
        Ok(())
    })
}

/// Asserts the pages load without console errors or uncaught exceptions.
fn case_console_clean<'a>(ctx: &'a CaseCtx) -> CaseFuture<'a> {
    Box::pin(async move {
        let mut problems = Vec::new();
        for path in [
            "/".to_string(),
            format!("/product/{}", seed::PRODUCT_SLUG),
            format!("/{}/{}", seed::POST_CATEGORY_SLUG, seed::POST_SLUG),
        ] {
            ctx.clear_browser_errors();
            ctx.goto(&path).await?;
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
