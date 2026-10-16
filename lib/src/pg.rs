//! Opt-in TLS for Postgres connection pools.
//!
//! Connections default to plain TCP (`NoTls`), matching the local/dev setup.
//! Set `PG_TLS` to a truthy value (`1`, `true`, `yes`, `on`) to connect with
//! rustls instead, so credentials and data are encrypted when Postgres is
//! remote.

use deadpool_postgres::tokio_postgres::NoTls;
use deadpool_postgres::{Config, Pool, Runtime};
use tokio_postgres_rustls::MakeRustlsConnect;

/// Whether Postgres TLS is enabled via the `PG_TLS` environment variable.
pub fn tls_enabled() -> bool {
    match std::env::var("PG_TLS") {
        Ok(v) => matches!(
            v.trim().to_ascii_lowercase().as_str(),
            "1" | "true" | "yes" | "on"
        ),
        Err(_) => false,
    }
}

/// Create a Postgres connection pool, with TLS when [`tls_enabled`].
///
/// Certificates are verified against the Mozilla root store (`webpki-roots`).
pub fn create_pool(config: &Config) -> Pool {
    if tls_enabled() {
        config
            .create_pool(
                Some(Runtime::Tokio1),
                MakeRustlsConnect::with_webpki_roots(),
            )
            .expect("failed to create pool")
    } else {
        config
            .create_pool(Some(Runtime::Tokio1), NoTls)
            .expect("failed to create pool")
    }
}
