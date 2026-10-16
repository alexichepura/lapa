//! The Lapa public site (Leptos SSR) under test.

use std::path::{Path, PathBuf};
use std::time::Duration;

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
                name: "draft-hidden-and-404",
                run: case_draft_hidden,
            },
            Case {
                name: "images-render",
                run: case_images,
            },
            Case {
                name: "meta-tags",
                run: case_meta_tags,
            },
            Case {
                name: "navigation",
                run: case_navigation,
            },
            Case {
                name: "robots-txt",
                run: case_robots_txt,
            },
            Case {
                name: "unknown-route-404",
                run: case_unknown_route,
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

/// `publish_at` in the future must hide a row from the lists *and* make its
/// detail page a 404 (the queries filter on `publish_at < NOW()`).
fn case_draft_hidden<'a>(ctx: &'a CaseCtx) -> CaseFuture<'a> {
    Box::pin(async move {
        ctx.goto("/").await?;
        expect(ctx.page.locator("text=E2E Draft Product H1"))
            .to_have_count(0)
            .await?;
        expect(ctx.page.locator("text=E2E Draft Post H1"))
            .to_have_count(0)
            .await?;

        let status = ctx
            .goto_status(&format!("/product/{}", seed::DRAFT_PRODUCT_SLUG))
            .await?;
        anyhow::ensure!(
            status == 404,
            "draft product page returned {status}, want 404"
        );
        expect(ctx.page.locator("text=Product not found."))
            .to_be_visible()
            .await?;

        let status = ctx
            .goto_status(&format!(
                "/{}/{}",
                seed::POST_CATEGORY_SLUG,
                seed::DRAFT_POST_SLUG
            ))
            .await?;
        anyhow::ensure!(status == 404, "draft post page returned {status}, want 404");
        expect(ctx.page.locator("text=Post not found."))
            .to_be_visible()
            .await?;
        Ok(())
    })
}

/// The image pipeline: the uploaded originals exist, the product hero (home
/// list + detail) and the in-content image are resized on demand by the CDN
/// handlers and must actually decode in the browser.
fn case_images<'a>(ctx: &'a CaseCtx) -> CaseFuture<'a> {
    Box::pin(async move {
        ctx.goto("/").await?;
        ctx.wait_for_image(".ProductList img", 60).await?;

        ctx.goto(&format!("/product/{}", seed::PRODUCT_SLUG))
            .await?;
        ctx.wait_for_image(".product-images img", 60).await?;
        ctx.wait_for_image("article figure img", 60).await?;

        let content_src: String = ctx
            .page
            .evaluate(
                "() => { const i = document.querySelector('article figure img'); \
                 return i ? i.getAttribute('src') : ''; }",
                None::<&()>,
            )
            .await?;
        anyhow::ensure!(
            content_src.starts_with("/cdn/"),
            "content image src is {content_src:?}, want a /cdn/ URL"
        );
        Ok(())
    })
}

fn case_meta_tags<'a>(ctx: &'a CaseCtx) -> CaseFuture<'a> {
    Box::pin(async move {
        ctx.goto("/").await?;
        let title: String = ctx.page.evaluate_value("document.title").await?;
        anyhow::ensure!(title == "Home - Lapa", "home title is {title:?}");

        ctx.goto(&format!("/product/{}", seed::PRODUCT_SLUG))
            .await?;
        let title: String = ctx.page.evaluate_value("document.title").await?;
        anyhow::ensure!(title == "E2E Product - Lapa", "product title is {title:?}");
        let description: String = ctx
            .page
            .evaluate(
                "() => { const m = document.querySelector('meta[name=\"description\"]'); \
                 return m ? m.content : ''; }",
                None::<&()>,
            )
            .await?;
        anyhow::ensure!(
            description == "E2E product meta description",
            "product meta description is {description:?}"
        );

        ctx.goto(&format!(
            "/{}/{}",
            seed::POST_CATEGORY_SLUG,
            seed::POST_SLUG
        ))
        .await?;
        let title: String = ctx.page.evaluate_value("document.title").await?;
        anyhow::ensure!(title == "E2E Post - Lapa", "post title is {title:?}");
        Ok(())
    })
}

/// Client-side routing: clicking a list item navigates without a full reload.
fn case_navigation<'a>(ctx: &'a CaseCtx) -> CaseFuture<'a> {
    Box::pin(async move {
        ctx.goto("/").await?;
        ctx.page.locator("text=E2E Product H1").click(None).await?;
        wait_for_url(ctx, &format!("/product/{}", seed::PRODUCT_SLUG), 15).await?;
        expect(ctx.page.locator("h1"))
            .to_contain_text("E2E Product H1")
            .await?;

        ctx.goto("/").await?;
        ctx.page.locator("text=E2E Post H1").click(None).await?;
        wait_for_url(
            ctx,
            &format!("/{}/{}", seed::POST_CATEGORY_SLUG, seed::POST_SLUG),
            15,
        )
        .await?;
        expect(ctx.page.locator("h1"))
            .to_contain_text("E2E Post H1")
            .await?;
        Ok(())
    })
}

fn case_robots_txt<'a>(ctx: &'a CaseCtx) -> CaseFuture<'a> {
    Box::pin(async move {
        let status = ctx.goto_status("/robots.txt").await?;
        anyhow::ensure!(status == 200, "robots.txt returned {status}, want 200");
        let content = ctx.page.content().await?;
        anyhow::ensure!(
            content.contains("User-agent: *"),
            "robots.txt body did not contain the seeded rules"
        );
        Ok(())
    })
}

/// An unmatched path falls through the axum fallback to the `ErrorTemplate`.
fn case_unknown_route<'a>(ctx: &'a CaseCtx) -> CaseFuture<'a> {
    Box::pin(async move {
        let status = ctx.goto_status("/definitely-not-a-real-page").await?;
        anyhow::ensure!(status == 404, "unknown route returned {status}, want 404");
        expect(ctx.page.locator("h1"))
            .to_contain_text("Error")
            .await?;
        let content = ctx.page.content().await?;
        anyhow::ensure!(
            content.contains("Not Found"),
            "error page did not mention Not Found"
        );
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

async fn wait_for_url(ctx: &CaseCtx, suffix: &str, secs: u64) -> anyhow::Result<()> {
    for _ in 0..secs * 4 {
        if ctx.page.url().ends_with(suffix) {
            return Ok(());
        }
        tokio::time::sleep(Duration::from_millis(250)).await;
    }
    anyhow::bail!("URL never reached *{suffix} (currently {})", ctx.page.url())
}
