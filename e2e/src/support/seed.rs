//! Deterministic fixtures. Fixed ids and timestamps keep screenshots stable.
//!
//! Lapa has no migration runner: the production flow is `psql < schema.sql`
//! (see the README), so the harness applies `schema.sql` and then a small seed.

use std::path::Path;

use anyhow::{Context, Result};

pub const PRODUCT_SLUG: &str = "e2e-product";
pub const POST_CATEGORY_SLUG: &str = "blog";
pub const POST_SLUG: &str = "e2e-post";
pub const HOME_TEXT: &str = "Welcome to the e2e home text.";

pub async fn apply(database_url: &str) -> Result<()> {
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
        .batch_execute(&seed_sql())
        .await
        .context("applying seed SQL")?;
    Ok(())
}

fn schema_path() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("schema.sql")
}

fn seed_sql() -> String {
    format!(
        r#"
-- The site unwraps the Settings row on every render (`settings_db`,
-- `settings_home`, `robots_txt`), so a row must exist.
INSERT INTO "Settings" (id, site_url, robots_txt, home_text)
VALUES ('e2e-settings', 'https://e2e.example', E'User-agent: *\nAllow: /', E'{HOME_TEXT}\n\nSecond paragraph.')
ON CONFLICT (id) DO NOTHING;

INSERT INTO "Content" (id, created_at, json) VALUES
    ('e2e-product-content', TIMESTAMP '2026-01-01 00:00:00',
     '[{{"type":"p","children":[{{"text":"E2E product body."}}]}}]'),
    ('e2e-post-content', TIMESTAMP '2026-01-01 00:00:00',
     '[{{"type":"p","children":[{{"text":"E2E post body."}}]}}]')
ON CONFLICT (id) DO NOTHING;

INSERT INTO "Product"
    (id, created_at, publish_at, slug, meta_title, meta_description, h1, content_id)
VALUES
    ('e2e-product', TIMESTAMP '2026-01-01 00:00:00', TIMESTAMP '2026-01-01 00:00:00',
     '{PRODUCT_SLUG}', 'E2E Product', 'E2E product meta description', 'E2E Product H1',
     'e2e-product-content')
ON CONFLICT (id) DO NOTHING;

INSERT INTO "PostCategory"
    (id, created_at, slug, name, meta_title, meta_description)
VALUES
    ('e2e-category', TIMESTAMP '2026-01-01 00:00:00', '{POST_CATEGORY_SLUG}', 'Blog',
     'Blog', 'Blog meta description')
ON CONFLICT (id) DO NOTHING;

INSERT INTO "Post"
    (id, created_at, publish_at, slug, meta_title, meta_description, h1, category_id, content_id)
VALUES
    ('e2e-post', TIMESTAMP '2026-01-01 00:00:00', TIMESTAMP '2026-01-01 00:00:00',
     '{POST_SLUG}', 'E2E Post', 'E2E post meta description', 'E2E Post H1',
     'e2e-category', 'e2e-post-content')
ON CONFLICT (id) DO NOTHING;
"#
    )
}
