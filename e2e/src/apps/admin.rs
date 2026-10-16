//! The Lapa admin UI (Leptos SSR + session auth) under test.

use std::path::{Path, PathBuf};
use std::time::Duration;

use playwright_rs::{SelectOption, expect};

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
            Case {
                name: "logout",
                run: case_logout,
            },
            Case {
                name: "auth-negatives",
                run: case_auth_negatives,
            },
            Case {
                name: "product-crud",
                run: case_product_crud,
            },
            Case {
                name: "category-post-crud",
                run: case_category_post_crud,
            },
            Case {
                name: "image-hero-and-order",
                run: case_image_hero_and_order,
            },
            Case {
                name: "image-upload",
                run: case_image_upload,
            },
            Case {
                name: "content-editor",
                run: case_content_editor,
            },
        ]
    }
}

/// Unauthenticated `/` must land on the login form.
fn case_login_page<'a>(ctx: &'a CaseCtx) -> CaseFuture<'a> {
    Box::pin(async move {
        logout_if_logged_in(ctx).await?;
        ctx.goto_for("admin", "/").await?;
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
            ctx.goto_for("admin", path).await?;
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

/// Logging out returns to the login form.
fn case_logout<'a>(ctx: &'a CaseCtx) -> CaseFuture<'a> {
    Box::pin(async move {
        login(ctx).await?;
        ctx.page.locator("text=Log Out").click(None).await?;
        wait_at_least(ctx, "text=Log in", 1, 15).await?;
        anyhow::ensure!(
            !dashboard_visible(ctx).await?,
            "still authenticated after logging out"
        );
        Ok(())
    })
}

/// Wrong credentials are rejected and protected routes/endpoints stay closed.
fn case_auth_negatives<'a>(ctx: &'a CaseCtx) -> CaseFuture<'a> {
    Box::pin(async move {
        logout_if_logged_in(ctx).await?;

        ctx.goto_for("admin", "/").await?;
        ctx.page
            .locator("input[name=username]")
            .fill(seed::ADMIN_USERNAME, None)
            .await?;
        ctx.page
            .locator("input[name=password]")
            .fill("definitely-not-the-password", None)
            .await?;
        submit(ctx, "Login").await?;
        wait_at_least(ctx, "text=Login-password pair not found", 1, 15).await?;

        // A protected route bounces to the login form.
        ctx.goto_for("admin", "/product").await?;
        wait_url(ctx, 15, |url| url.ends_with("/login")).await?;

        // The `/api` server-fn router is default-deny while logged out
        // (`server_fn_private` returns 404 before dispatching).
        let status: u16 = ctx
            .page
            .evaluate(
                "async () => { const r = await fetch('/api/get_stats', { method: 'POST' }); \
                 return r.status; }",
                None::<&()>,
            )
            .await?;
        anyhow::ensure!(
            status == 404,
            "unauthenticated /api call returned {status}, want 404"
        );
        Ok(())
    })
}

/// Create a product, edit it and delete it through the admin UI.
fn case_product_crud<'a>(ctx: &'a CaseCtx) -> CaseFuture<'a> {
    Box::pin(async move {
        login(ctx).await?;

        ctx.goto_for("admin", "/product/new").await?;
        ctx.page
            .locator("input[name=slug]")
            .fill("e2e-crud-product", None)
            .await?;
        ctx.page
            .locator("input[name=meta_title]")
            .fill("E2E CRUD Product", None)
            .await?;
        ctx.page
            .locator("input[name=meta_description]")
            .fill("E2E CRUD description", None)
            .await?;
        ctx.page
            .locator("input[name=h1]")
            .fill("E2E CRUD H1", None)
            .await?;
        submit(ctx, "Create product draft").await?;

        // On success the create action navigates to `/product/{id}` and the
        // edit form (its submit button) appears.
        wait_at_least(
            ctx,
            "input[type=submit][value=\"Submit product data\"]",
            1,
            20,
        )
        .await?;
        let id = ctx
            .page
            .url()
            .rsplit('/')
            .next()
            .unwrap_or_default()
            .to_string();
        anyhow::ensure!(
            !id.is_empty(),
            "could not read the new product id from the URL"
        );

        // Edit.
        ctx.page
            .locator("input[name=h1]")
            .fill("E2E CRUD H1 (edited)", None)
            .await?;
        submit(ctx, "Submit product data").await?;
        wait_at_least(ctx, ".Alert.Success", 1, 20).await?;

        ctx.goto_for("admin", "/product").await?;
        expect(ctx.page.locator("text=E2E CRUD H1 (edited)"))
            .to_be_visible()
            .await?;

        // Delete (the danger-zone input must match the slug to enable submit).
        ctx.goto_for("admin", &format!("/product/{id}")).await?;
        ctx.page
            .locator(
                "fieldset:has-text(\"Danger zone\") input:not([type=submit]):not([type=hidden])",
            )
            .fill("e2e-crud-product", None)
            .await?;
        submit(ctx, "Delete product").await?;
        wait_url(ctx, 20, |url| url.ends_with("/product")).await?;

        ctx.goto_for("admin", "/product").await?;
        expect(ctx.page.locator("text=E2E CRUD H1 (edited)"))
            .to_have_count(0)
            .await?;
        Ok(())
    })
}

/// Create a category, then a post that references it, and see it listed.
fn case_category_post_crud<'a>(ctx: &'a CaseCtx) -> CaseFuture<'a> {
    Box::pin(async move {
        login(ctx).await?;

        ctx.goto_for("admin", "/post-category/new").await?;
        ctx.page
            .locator("input[name=slug]")
            .fill("e2e-cat-crud", None)
            .await?;
        ctx.page
            .locator("input[name=name]")
            .fill("E2E Cat", None)
            .await?;
        submit(ctx, "Create category").await?;
        wait_url(ctx, 20, |url| {
            url.contains("/post-category/") && !url.ends_with("/new")
        })
        .await?;

        ctx.goto_for("admin", "/post/create").await?;
        ctx.page
            .locator("select[name=category_id]")
            .select_option(SelectOption::Label("E2E Cat".into()), None)
            .await?;
        ctx.page
            .locator("input[name=slug]")
            .fill("e2e-post-crud", None)
            .await?;
        ctx.page
            .locator("input[name=meta_title]")
            .fill("E2E CRUD Post", None)
            .await?;
        ctx.page
            .locator("input[name=meta_description]")
            .fill("E2E CRUD post description", None)
            .await?;
        ctx.page
            .locator("input[name=h1]")
            .fill("E2E CRUD Post H1", None)
            .await?;
        submit(ctx, "Create post draft").await?;
        wait_url(ctx, 20, |url| {
            url.contains("/post/") && !url.ends_with("/create")
        })
        .await?;

        ctx.goto_for("admin", "/post").await?;
        expect(ctx.page.locator("text=E2E CRUD Post H1"))
            .to_be_visible()
            .await?;
        Ok(())
    })
}

/// Make a non-hero image the hero, then reorder it and confirm both persist.
fn case_image_hero_and_order<'a>(ctx: &'a CaseCtx) -> CaseFuture<'a> {
    Box::pin(async move {
        login(ctx).await?;
        ctx.goto_for("admin", &format!("/product/{}", seed::PRODUCT_SLUG))
            .await?;
        wait_at_least(ctx, ".images figure", 2, 20).await?;

        // Swap the hero to the second image.
        ctx.page
            .locator(
                ".images figure:has-text(\"E2E product image 2\") button:has-text(\"Make hero\")",
            )
            .click(None)
            .await?;
        wait_at_least(
            ctx,
            ".images figure:has-text(\"E2E product image 2\") button:disabled:has-text(\"Hero\")",
            1,
            20,
        )
        .await?;

        // Move the first image below the second, then persist the order.
        ctx.page
            .locator(".images figure")
            .first()
            .locator("button")
            .last()
            .click(None)
            .await?;
        submit(ctx, "Save order").await?;
        wait_at_least(ctx, ".Alert.Success", 1, 20).await?;

        // A fresh load reads the persisted order: image 2 is now first.
        ctx.goto_for("admin", &format!("/product/{}", seed::PRODUCT_SLUG))
            .await?;
        expect(
            ctx.page
                .locator(".images figure")
                .first()
                .locator("figcaption"),
        )
        .to_contain_text("E2E product image 2")
        .await?;
        Ok(())
    })
}

/// The Slate rich-text editor loads the seeded content in the product page.
fn case_content_editor<'a>(ctx: &'a CaseCtx) -> CaseFuture<'a> {
    Box::pin(async move {
        login(ctx).await?;
        ctx.clear_browser_errors();
        ctx.goto_for("admin", &format!("/product/{}", seed::PRODUCT_SLUG))
            .await?;
        // Slate mounts a contenteditable inside `#app` once hydrated.
        wait_at_least(ctx, "#app [contenteditable]", 1, 30).await?;
        // The seeded product body is rendered into the editor.
        wait_at_least(ctx, "text=E2E product body.", 1, 30).await?;

        let problems: Vec<String> = ctx
            .console_errors()
            .into_iter()
            .filter(|m| !CaseCtx::is_benign(m))
            .chain(
                ctx.page_errors()
                    .into_iter()
                    .map(|e| e.lines().next().unwrap_or_default().to_string()),
            )
            .collect();
        anyhow::ensure!(
            problems.is_empty(),
            "the editor logged errors:\n  {}",
            problems.join("\n  ")
        );
        Ok(())
    })
}

/// Upload an image to a product through the admin file input.
fn case_image_upload<'a>(ctx: &'a CaseCtx) -> CaseFuture<'a> {
    Box::pin(async move {
        login(ctx).await?;
        ctx.goto_for("admin", &format!("/product/{}", seed::PRODUCT_SLUG))
            .await?;

        let file = Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures/tiny.png");
        ctx.page
            .locator("input[type=file]")
            .set_input_files(&file, None)
            .await?;
        ctx.page
            .locator("input[name=alt]")
            .fill("Uploaded e2e image", None)
            .await?;
        // The form submits the file as a real multipart field.
        submit(ctx, "Upload image").await?;

        // The images list refetches on upload; the new alt shows up.
        wait_at_least(ctx, "text=Uploaded e2e image", 1, 20).await?;
        Ok(())
    })
}

// --- helpers -------------------------------------------------------------
/// Log in through the form as the seeded admin, or no-op when already
/// authenticated (cases share one browser context).
pub(crate) async fn login(ctx: &CaseCtx) -> anyhow::Result<()> {
    login_as(ctx, seed::ADMIN_USERNAME, seed::ADMIN_PASSWORD).await
}

/// Log in as a specific user, or no-op when already authenticated.
pub(crate) async fn login_as(ctx: &CaseCtx, username: &str, password: &str) -> anyhow::Result<()> {
    ctx.goto_for("admin", "/").await?;
    if dashboard_visible(ctx).await? {
        return Ok(());
    }

    ctx.page
        .locator("input[name=username]")
        .fill(username, None)
        .await?;
    ctx.page
        .locator("input[name=password]")
        .fill(password, None)
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

pub(crate) async fn logout_if_logged_in(ctx: &CaseCtx) -> anyhow::Result<()> {
    ctx.goto_for("admin", "/").await?;
    if dashboard_visible(ctx).await? {
        ctx.page.locator("text=Log Out").click(None).await?;
        wait_at_least(ctx, "text=Log in", 1, 15).await?;
    }
    Ok(())
}

/// Click a `<input type=submit>` by its `value`.
async fn submit(ctx: &CaseCtx, value: &str) -> anyhow::Result<()> {
    let selector = format!("input[type=submit][value={value:?}]");
    ctx.page.locator(selector).click(None).await?;
    Ok(())
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

/// Poll until `selector` matches at least `n` elements (`count` is not strict).
async fn wait_at_least(ctx: &CaseCtx, selector: &str, n: usize, secs: u64) -> anyhow::Result<()> {
    for _ in 0..secs * 4 {
        if ctx.page.locator(selector).count().await.unwrap_or(0) >= n {
            return Ok(());
        }
        tokio::time::sleep(Duration::from_millis(250)).await;
    }
    anyhow::bail!("{selector} never reached {n} element(s) within {secs}s")
}

async fn wait_url(ctx: &CaseCtx, secs: u64, pred: impl Fn(&str) -> bool) -> anyhow::Result<()> {
    for _ in 0..secs * 4 {
        if pred(&ctx.page.url()) {
            return Ok(());
        }
        tokio::time::sleep(Duration::from_millis(250)).await;
    }
    anyhow::bail!("URL never matched (currently {})", ctx.page.url())
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
