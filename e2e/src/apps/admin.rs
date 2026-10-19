//! The Lapa admin UI (Leptos SSR + session auth) under test.

use std::path::{Path, PathBuf};
use std::time::Duration;

use playwright_rs::{SelectOption, expect};

use crate::apps::{AppUnderTest, Case, CaseCtx, CaseFuture};
use crate::support::seed;

/// A user dedicated to the disabled-account case (see `case_disabled_user`) so
/// toggling `active` never touches the shared seeded admin.
const DEACTIVATABLE_USERNAME: &str = "e2e-deactivatable";
const DEACTIVATABLE_PASSWORD: &str = "e2e-deactivatable-password";

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
                name: "login-remember",
                run: case_login_remember,
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
            Case {
                name: "disabled-user",
                run: case_disabled_user,
            },
            Case {
                name: "db-error-500",
                run: case_db_error_returns_500,
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
        // screenshot is deterministic. The class name is stylance-hashed
        // (`routingProgress-<hash>`), so match the stable name as a substring.
        wait_until_hidden(ctx, "[class*=\"routingProgress\"]", 10).await?;
        expect(ctx.page.locator("text=All time"))
            .to_be_visible()
            .await?;
        ctx.snapshot("dashboard").await?;
        Ok(())
    })
}

/// "Remember me?" gives the session a long-lived expiry; the default is a
/// browser-session cookie.
fn case_login_remember<'a>(ctx: &'a CaseCtx) -> CaseFuture<'a> {
    Box::pin(async move {
        logout_if_logged_in(ctx).await?;

        ctx.goto_for("admin", "/").await?;
        ctx.page
            .locator("input[name=username]")
            .fill(seed::ADMIN_USERNAME, None)
            .await?;
        ctx.page
            .locator("input[name=password]")
            .fill(seed::ADMIN_PASSWORD, None)
            .await?;
        ctx.page.locator("input[name=remember]").click(None).await?;
        submit(ctx, "Login").await?;

        wait_dashboard(ctx).await?;

        // `Remember me?` sets a ~30 day expiry; without it the record uses the
        // 2-week default. Assert the persisted record is in the longer window.
        ctx.exec_sql(
            "DO $$ BEGIN \
                 IF NOT EXISTS (SELECT 1 FROM \"Session\" \
                     WHERE expires > now() + interval '29 days') \
                 THEN RAISE EXCEPTION 'remember-me session did not get a long expiry'; \
                 END IF; \
             END $$;",
        )
        .await?;

        logout_if_logged_in(ctx).await?;
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
        wait_at_least(ctx, "text=Success", 1, 20).await?;

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
        // The image grid lives in the "Images" fieldset; class names are hashed
        // by stylance, so anchor on the legend text instead.
        let figures = "fieldset:has-text(\"Images\") figure";
        wait_at_least(ctx, figures, 2, 20).await?;

        // Swap the hero to the second image.
        ctx.page
            .locator(format!(
                "{figures}:has-text(\"E2E product image 2\") button:has-text(\"Make hero\")"
            ))
            .click(None)
            .await?;
        wait_at_least(
            ctx,
            &format!(
                "{figures}:has-text(\"E2E product image 2\") button:disabled:has-text(\"Hero\")"
            ),
            1,
            20,
        )
        .await?;

        // Move the first image below the second, then persist the order.
        ctx.page
            .locator(figures)
            .first()
            .locator("button")
            .last()
            .click(None)
            .await?;
        submit(ctx, "Save order").await?;
        wait_at_least(ctx, "text=Success", 1, 20).await?;

        // A fresh load reads the persisted order: image 2 is now first.
        ctx.goto_for("admin", &format!("/product/{}", seed::PRODUCT_SLUG))
            .await?;
        expect(ctx.page.locator(figures).first().locator("figcaption"))
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

        // Issue #26: the upload gets `order = current count`, so after a fresh
        // load it is the last image in the grid.
        ctx.goto_for("admin", &format!("/product/{}", seed::PRODUCT_SLUG))
            .await?;
        let figures = "fieldset:has-text(\"Images\") figure";
        wait_at_least(ctx, figures, 3, 20).await?;
        expect(ctx.page.locator(figures).last().locator("figcaption"))
            .to_contain_text("Uploaded e2e image")
            .await?;
        Ok(())
    })
}

/// A disabled account loses access: existing sessions fail closed, new logins
/// are rejected, and re-enabling restores access.
fn case_disabled_user<'a>(ctx: &'a CaseCtx) -> CaseFuture<'a> {
    Box::pin(async move {
        // A dedicated user keeps this case from touching the shared seed, and a
        // clean slate avoids depending on the previous case's auth state.
        logout_if_logged_in(ctx).await?;

        let hash = bcrypt::hash(DEACTIVATABLE_PASSWORD, 4)?;
        ctx.exec_sql(&format!(
            "INSERT INTO \"User\" (id, username, password, deactivated_at) \
             VALUES ('e2e-deactivatable', '{DEACTIVATABLE_USERNAME}', '{hash}', NULL) \
             ON CONFLICT (id) DO UPDATE \
             SET password = EXCLUDED.password, deactivated_at = NULL"
        ))
        .await?;

        // While active the account behaves normally.
        login_as(ctx, DEACTIVATABLE_USERNAME, DEACTIVATABLE_PASSWORD).await?;
        expect(ctx.page.locator("h1"))
            .to_contain_text("Dashboard")
            .await?;

        // Disable it; the already-issued session must be rejected on the next
        // request, so `/` bounces back to the login form.
        ctx.exec_sql(&format!(
            "UPDATE \"User\" SET deactivated_at = now() \
             WHERE username = '{DEACTIVATABLE_USERNAME}'"
        ))
        .await?;
        ctx.goto_for("admin", "/").await?;
        wait_url(ctx, 15, |url| url.ends_with("/login")).await?;
        expect(ctx.page.locator("input[name=username]"))
            .to_be_visible()
            .await?;

        // The `/api` router is default-deny for the stale session too.
        let status = api_status(ctx, "get_stats").await?;
        anyhow::ensure!(
            status == 404,
            "disabled user's /api call returned {status}, want 404"
        );

        // And it cannot obtain a fresh session: same opaque error as a bad
        // password.
        ctx.page
            .locator("input[name=username]")
            .fill(DEACTIVATABLE_USERNAME, None)
            .await?;
        ctx.page
            .locator("input[name=password]")
            .fill(DEACTIVATABLE_PASSWORD, None)
            .await?;
        submit(ctx, "Login").await?;
        wait_at_least(ctx, "text=Login-password pair not found", 1, 15).await?;

        // Re-enabling restores access.
        ctx.exec_sql(&format!(
            "UPDATE \"User\" SET deactivated_at = NULL WHERE username = '{DEACTIVATABLE_USERNAME}'"
        ))
        .await?;
        login_as(ctx, DEACTIVATABLE_USERNAME, DEACTIVATABLE_PASSWORD).await?;
        expect(ctx.page.locator("h1"))
            .to_contain_text("Dashboard")
            .await?;

        // Leave the shared context logged out for the cases that follow.
        logout_if_logged_in(ctx).await?;
        Ok(())
    })
}

/// A database failure while loading the session yields a 500, not a silent
/// logout (axum-login maps `AuthnBackend` errors to 500).
fn case_db_error_returns_500<'a>(ctx: &'a CaseCtx) -> CaseFuture<'a> {
    Box::pin(async move {
        login(ctx).await?;

        // The intentional failure is logged at ERROR level; allow those lines so
        // the suite's fatal-log sweep does not flag them.
        ctx.allow_server_error("could not create auth session from session");
        ctx.allow_server_error("Status code: 500 Internal Server Error");

        // Make `get_user` fail by hiding the table while the session loads.
        ctx.exec_sql("ALTER TABLE \"User\" RENAME TO \"User_unavailable\"")
            .await?;
        let status = ctx.goto_status_for("admin", "/").await;
        // Always restore before asserting so a failure cannot poison later runs.
        let restored = ctx
            .exec_sql("ALTER TABLE \"User_unavailable\" RENAME TO \"User\"")
            .await;
        restored.expect("restoring the User table");
        let status = status?;
        anyhow::ensure!(
            status == 500,
            "DB failure during session load returned {status}, want 500"
        );

        // The session survives the blip.
        ctx.goto_for("admin", "/").await?;
        expect(ctx.page.locator("h1"))
            .to_contain_text("Dashboard")
            .await?;
        logout_if_logged_in(ctx).await?;
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

/// Log in as a specific user expecting failure: asserts the opaque
/// credentials error and that the dashboard stays out of reach.
pub(crate) async fn login_as_expect_failure(
    ctx: &CaseCtx,
    username: &str,
    password: &str,
) -> anyhow::Result<()> {
    ctx.goto_for("admin", "/").await?;
    anyhow::ensure!(
        !dashboard_visible(ctx).await?,
        "unexpectedly authenticated as {username}"
    );
    ctx.page
        .locator("input[name=username]")
        .fill(username, None)
        .await?;
    ctx.page
        .locator("input[name=password]")
        .fill(password, None)
        .await?;
    submit(ctx, "Login").await?;
    wait_at_least(ctx, "text=Login-password pair not found", 1, 15).await
}

pub(crate) async fn logout_if_logged_in(ctx: &CaseCtx) -> anyhow::Result<()> {
    ctx.goto_for("admin", "/").await?;
    if dashboard_visible(ctx).await? {
        ctx.page.locator("text=Log Out").click(None).await?;
        wait_at_least(ctx, "text=Log in", 1, 15).await?;
    }
    Ok(())
}

/// POST to a `/api` server function from the page and return the HTTP status.
/// The body is intentionally empty: while logged out the `server_fn_private`
/// guard rejects the request (404) before it is dispatched, so the body is
/// never parsed. Only meaningful for the default-deny cases.
async fn api_status(ctx: &CaseCtx, fn_name: &str) -> anyhow::Result<u16> {
    let expr = format!(
        "async () => {{ const r = await fetch('/api/{fn_name}', {{ method: 'POST' }}); \
         return r.status; }}"
    );
    Ok(ctx.page.evaluate(&expr, None::<&()>).await?)
}

/// Click a `<input type=submit>` by its `value`.
async fn submit(ctx: &CaseCtx, value: &str) -> anyhow::Result<()> {
    let selector = format!("input[type=submit][value={value:?}]");
    ctx.page.locator(selector).click(None).await?;
    Ok(())
}

/// Poll until the dashboard heading is rendered.
async fn wait_dashboard(ctx: &CaseCtx) -> anyhow::Result<()> {
    for _ in 0..80 {
        if dashboard_visible(ctx).await? {
            return Ok(());
        }
        tokio::time::sleep(Duration::from_millis(250)).await;
    }
    anyhow::bail!("never reached the dashboard (url={})", ctx.page.url())
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
