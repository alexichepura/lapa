//! Applies the embedded migrations to a throwaway database.
//!
//! Set `TEST_DATABASE_URL` (e.g. an empty `lapa_test` database) to run; the
//! test is skipped when it is unset, so `cargo test` stays hermetic.

use db::migrate;
use deadpool_postgres::tokio_postgres::{self, NoTls};
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod, Runtime};

#[tokio::test]
async fn applies_embedded_migrations() {
    let Ok(url) = std::env::var("TEST_DATABASE_URL") else {
        eprintln!("TEST_DATABASE_URL not set, skipping");
        return;
    };
    let pg_config = url.parse::<tokio_postgres::Config>().expect("invalid url");
    let manager = Manager::from_config(
        pg_config,
        NoTls,
        ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        },
    );
    let pool = Pool::builder(manager)
        .max_size(2)
        .runtime(Runtime::Tokio1)
        .build()
        .expect("failed to build pool");

    migrate(&pool).await.expect("failed to run migrations");

    let client = pool.get().await.expect("failed to get connection");
    let count: i64 = client
        .query_one("SELECT COUNT(*) FROM refinery_schema_history", &[])
        .await
        .expect("missing refinery_schema_history")
        .get(0);
    assert!(count >= 3, "expected all migrations to be applied");

    let tables: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM information_schema.tables \
             WHERE table_schema = 'public' AND table_name = 'Product'",
            &[],
        )
        .await
        .expect("query failed")
        .get(0);
    assert_eq!(tables, 1, "Product table was not created");

    // V2: `User.deactivated_at` exists and defaults to NULL (active), so
    // existing users stay enabled after the upgrade.
    client
        .execute(
            "INSERT INTO \"User\" (id, username, password) \
             VALUES ('migrate-user', 'migrate-user', 'x')",
            &[],
        )
        .await
        .expect("inserting a user");
    let active: bool = client
        .query_one(
            "SELECT deactivated_at IS NULL FROM \"User\" WHERE id = 'migrate-user'",
            &[],
        )
        .await
        .expect("selecting deactivated_at")
        .get(0);
    assert!(
        active,
        "new users should be active (deactivated_at IS NULL)"
    );

    // V3: `Session.user_id` exists (used to purge a user's sessions).
    let session_user_id_columns: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM information_schema.columns \
             WHERE table_name = 'Session' AND column_name = 'user_id'",
            &[],
        )
        .await
        .expect("query failed")
        .get(0);
    assert_eq!(
        session_user_id_columns, 1,
        "Session.user_id was not created"
    );
}
