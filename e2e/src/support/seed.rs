//! Deterministic fixtures. Fixed ids and timestamps keep screenshots stable.
//!
//! Production applies the refinery migrations in `db/migrations` via
//! `cargo cli migrate` (see the README); the harness applies the equivalent
//! `schema.sql` directly and then a small seed.

use std::path::Path;

use anyhow::{Context, Result};

pub const PRODUCT_SLUG: &str = "e2e-product";
pub const POST_CATEGORY_SLUG: &str = "blog";
pub const POST_SLUG: &str = "e2e-post";
/// Published in the future, so it must never show up in a list or page.
pub const DRAFT_PRODUCT_SLUG: &str = "e2e-product-draft";
pub const DRAFT_POST_SLUG: &str = "e2e-post-draft";
pub const HOME_TEXT: &str = "Welcome to the e2e home text.";

/// Image ids (must not contain `_`: the CDN handler splits `<id>_<size>`).
pub const PRODUCT_IMAGE_ID: &str = "e2e-product-image";
pub const PRODUCT_IMAGE_ID_2: &str = "e2e-product-image-2";
pub const CONTENT_IMAGE_ID: &str = "e2e-content-image";

pub const ADMIN_USERNAME: &str = "e2e-admin";
pub const ADMIN_PASSWORD: &str = "e2e-password";

const TS: &str = "TIMESTAMP '2026-01-01 00:00:00'";
const TS_FUTURE: &str = "TIMESTAMP '2030-01-01 00:00:00'";
const PRODUCT_CONTENT_ID: &str = "e2e-product-content";
const POST_CONTENT_ID: &str = "e2e-post-content";
const POST_DRAFT_CONTENT_ID: &str = "e2e-post-draft-content";

pub async fn apply(database_url: &str, image_upload_dir: &Path) -> Result<()> {
    write_image_fixtures(image_upload_dir)?;

    let password_hash = bcrypt::hash(ADMIN_PASSWORD, 4).context("hashing the admin password")?;

    let (client, connection) = tokio_postgres::connect(database_url, tokio_postgres::NoTls)
        .await
        .context("connecting to seeded Postgres")?;
    tokio::spawn(async move {
        let _ = connection.await;
    });

    let schema = schema_path();
    let schema_sql = std::fs::read_to_string(&schema)
        .with_context(|| format!("reading {}", schema.display()))?;
    client
        .batch_execute(&schema_sql)
        .await
        .context("applying schema.sql")?;

    client
        .batch_execute(&seed_sql(&password_hash))
        .await
        .context("applying seed SQL")?;
    Ok(())
}

fn schema_path() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("schema.sql")
}

/// Drop the committed 1x1 PNG into the upload tree under the ids the seed rows
/// reference, so `/product-image/…` and `/cdn/…` have an original to resize.
fn write_image_fixtures(upload_dir: &Path) -> Result<()> {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures/tiny.png");
    let bytes = std::fs::read(&fixture)
        .with_context(|| format!("reading fixture {}", fixture.display()))?;
    for (subdir, name) in [
        ("product", PRODUCT_IMAGE_ID),
        ("product", PRODUCT_IMAGE_ID_2),
        ("content", CONTENT_IMAGE_ID),
    ] {
        let dir = upload_dir.join(subdir);
        std::fs::create_dir_all(&dir).with_context(|| format!("creating {}", dir.display()))?;
        std::fs::write(dir.join(format!("{name}.png")), &bytes)
            .with_context(|| format!("writing fixture for {name}"))?;
    }
    Ok(())
}

fn seed_sql(password_hash: &str) -> String {
    format!(
        r#"
-- The site unwraps the Settings row on every render (`settings_db`,
-- `settings_home`, `robots_txt`), so a row must exist.
INSERT INTO "Settings" (id, site_url, robots_txt, home_text)
VALUES ('e2e-settings', 'https://e2e.example', E'User-agent: *\nAllow: /', E'{HOME_TEXT}\n\nSecond paragraph.')
ON CONFLICT (id) DO NOTHING;

-- The admin login flow uses this user (password hashed by bcrypt at boot).
INSERT INTO "User" (id, created_at, username, password)
VALUES ('e2e-user', {TS}, '{ADMIN_USERNAME}', '{password_hash}')
ON CONFLICT (id) DO NOTHING;

INSERT INTO "Content" (id, created_at, json) VALUES
    ('{PRODUCT_CONTENT_ID}', {TS},
     '[{{"type":"p","children":[{{"text":"E2E product body."}}]}},{{"type":"img","id":"{CONTENT_IMAGE_ID}","alt":"E2E content image","caption":"E2E caption","children":[{{"text":""}}]}}]'),
    ('{POST_CONTENT_ID}', {TS},
     '[{{"type":"p","children":[{{"text":"E2E post body."}}]}}]'),
    ('{POST_DRAFT_CONTENT_ID}', {TS},
     '[{{"type":"p","children":[{{"text":"E2E post body."}}]}}]')
ON CONFLICT (id) DO NOTHING;

INSERT INTO "ContentImage" (id, created_at, alt, ext, content_id)
VALUES ('{CONTENT_IMAGE_ID}', {TS}, 'E2E content image', 'png', '{PRODUCT_CONTENT_ID}')
ON CONFLICT (id) DO NOTHING;

INSERT INTO "Product"
    (id, created_at, publish_at, slug, meta_title, meta_description, h1, content_id)
VALUES
    ('e2e-product', {TS}, {TS}, '{PRODUCT_SLUG}', 'E2E Product',
     'E2E product meta description', 'E2E Product H1', '{PRODUCT_CONTENT_ID}'),
    ('e2e-product-draft', {TS}, {TS_FUTURE}, '{DRAFT_PRODUCT_SLUG}', 'E2E Draft Product',
     'E2E draft product description', 'E2E Draft Product H1', '{PRODUCT_CONTENT_ID}')
ON CONFLICT (id) DO NOTHING;

INSERT INTO "ProductImage"
    (id, created_at, alt, ext, is_hero, "order", product_id)
VALUES
    ('{PRODUCT_IMAGE_ID}', {TS}, 'E2E product image', 'png', true, 0, 'e2e-product'),
    ('{PRODUCT_IMAGE_ID_2}', {TS}, 'E2E product image 2', 'png', false, 1, 'e2e-product')
ON CONFLICT (id) DO NOTHING;

INSERT INTO "PostCategory"
    (id, created_at, slug, name, meta_title, meta_description)
VALUES
    ('e2e-category', {TS}, '{POST_CATEGORY_SLUG}', 'Blog', 'Blog', 'Blog meta description')
ON CONFLICT (id) DO NOTHING;

INSERT INTO "Post"
    (id, created_at, publish_at, slug, meta_title, meta_description, h1, category_id, content_id)
VALUES
    ('e2e-post', {TS}, {TS}, '{POST_SLUG}', 'E2E Post',
     'E2E post meta description', 'E2E Post H1', 'e2e-category', '{POST_CONTENT_ID}'),
    ('e2e-post-draft', {TS}, {TS_FUTURE}, '{DRAFT_POST_SLUG}', 'E2E Draft Post',
     'E2E draft post description', 'E2E Draft Post H1', 'e2e-category', '{POST_DRAFT_CONTENT_ID}')
ON CONFLICT (id) DO NOTHING;
"#
    )
}
