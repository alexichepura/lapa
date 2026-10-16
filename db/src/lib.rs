//! Database schema migrations.
//!
//! Migrations live in `db/migrations` and are applied with
//! [refinery](https://docs.rs/refinery) (embedded in the `cli` binary, run
//! with `cargo cli migrate`). The previous production flow was
//! `psql < schema.sql`; `db/migrations/V1__init.sql` mirrors that schema.
//!
//! To add a migration, create the next file, e.g. `V2__add_thing.sql`, and run
//! `cargo cli migrate`.

use deadpool_postgres::Pool;

mod embedded {
    refinery::embed_migrations!("migrations");
}

#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("connection pool error: {0}")]
    Pool(#[from] deadpool_postgres::PoolError),
    #[error("migration error: {0}")]
    Migration(#[from] refinery::Error),
}

pub type DbResult<T> = Result<T, DbError>;

/// Apply every pending migration to the database behind `pool`.
pub async fn migrate(pool: &Pool) -> DbResult<()> {
    let mut client = pool.get().await?;
    let report = embedded::migrations::runner()
        .run_async(&mut **client)
        .await?;
    tracing::info!(
        applied = report.applied_migrations().len(),
        "database migrations applied"
    );
    Ok(())
}
