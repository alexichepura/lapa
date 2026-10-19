use std::collections::HashMap;

use async_trait::async_trait;
use chrono::DateTime;
use cornucopia::deadpool_postgres::{Object, Pool};
use cornucopia::queries;
use time::OffsetDateTime;
use tower_sessions::{
    Expiry, SessionManagerLayer, SessionStore,
    session::{Id, Record},
    session_store::{self, ExpiredDeletion},
};

/// The `axum-login` session data key (the builder's default).
const AUTH_DATA_KEY: &str = "axum-login.data";

/// Build the Postgres-backed session middleware layer.
pub fn session_layer(pool: &Pool) -> SessionManagerLayer<PostgresStore> {
    SessionManagerLayer::new(PostgresStore::new(pool.clone()))
        .with_name("session")
        .with_secure(session_cookie_secure())
        // Default to a browser-session cookie; the login flow extends this per
        // session when "Remember me?" is checked (`Session::set_expiry`).
        .with_expiry(Expiry::OnSessionEnd)
}

/// Whether the session cookie carries the `Secure` flag.
///
/// Default to on for release builds and off for debug builds (local dev and the
/// e2e harness serve over `http://127.0.0.1`). Override with `SESSION_SECURE`.
fn session_cookie_secure() -> bool {
    match std::env::var("SESSION_SECURE") {
        Ok(v) => matches!(
            v.trim().to_ascii_lowercase().as_str(),
            "1" | "true" | "yes" | "on"
        ),
        Err(_) => !cfg!(debug_assertions),
    }
}

#[derive(Debug, Clone)]
pub struct PostgresStore {
    pool: Pool,
}

impl PostgresStore {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    async fn db(&self) -> session_store::Result<Object> {
        self.pool
            .get()
            .await
            .map_err(|e| session_store::Error::Backend(e.to_string()))
    }
}

/// The `axum-login` user id embedded in the session data, if any.
fn session_user_id(record: &Record) -> Option<String> {
    record
        .data
        .get(AUTH_DATA_KEY)
        .and_then(|data| data.get("user_id"))
        .and_then(|id| id.as_str())
        .map(str::to_owned)
}

/// `time::OffsetDateTime` (tower-sessions) -> the `TIMESTAMP` column (chrono).
fn to_db_time(expiry: OffsetDateTime) -> chrono::NaiveDateTime {
    DateTime::from_timestamp(expiry.unix_timestamp(), 0)
        .unwrap_or_default()
        .naive_utc()
}

/// The `TIMESTAMP` column (chrono) -> `time::OffsetDateTime` (tower-sessions).
fn from_db_time(expires: chrono::NaiveDateTime) -> OffsetDateTime {
    OffsetDateTime::from_unix_timestamp(expires.and_utc().timestamp())
        .unwrap_or(OffsetDateTime::UNIX_EPOCH)
}

#[async_trait]
impl SessionStore for PostgresStore {
    async fn create(&self, record: &mut Record) -> session_store::Result<()> {
        // IDs are random i128s, so collisions are not a practical concern; a
        // unique violation would surface as a backend error via `save`.
        self.save(record).await
    }

    async fn save(&self, record: &Record) -> session_store::Result<()> {
        let db = self.db().await?;
        let data = serde_json::to_string(&record.data)
            .map_err(|e| session_store::Error::Encode(e.to_string()))?;
        let user_id = session_user_id(record);
        queries::session::store()
            .bind(
                &db,
                &record.id.to_string(),
                &data,
                &to_db_time(record.expiry_date),
                &user_id,
            )
            .await
            .map_err(|e| session_store::Error::Backend(e.to_string()))?;
        Ok(())
    }

    async fn load(&self, session_id: &Id) -> session_store::Result<Option<Record>> {
        let db = self.db().await?;
        let row = queries::session::load()
            .bind(&db, &session_id.to_string())
            .opt()
            .await
            .map_err(|e| session_store::Error::Backend(e.to_string()))?;
        let Some(row) = row else {
            return Ok(None);
        };
        let data: HashMap<String, serde_json::Value> = serde_json::from_str(&row.session)
            .map_err(|e| session_store::Error::Decode(e.to_string()))?;
        Ok(Some(Record {
            id: *session_id,
            data,
            expiry_date: from_db_time(row.expires),
        }))
    }

    async fn delete(&self, session_id: &Id) -> session_store::Result<()> {
        let db = self.db().await?;
        queries::session::delete_one_by_id()
            .bind(&db, &session_id.to_string())
            .await
            .map_err(|e| session_store::Error::Backend(e.to_string()))?;
        Ok(())
    }
}

#[async_trait]
impl ExpiredDeletion for PostgresStore {
    async fn delete_expired(&self) -> session_store::Result<()> {
        let db = self.db().await?;
        queries::session::delete_expired()
            .bind(&db)
            .await
            .map_err(|e| session_store::Error::Backend(e.to_string()))?;
        Ok(())
    }
}
