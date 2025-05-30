use leptos::prelude::{use_context, ServerFnError};
use clorinde::deadpool_postgres::{Config, CreatePoolError, Object, Pool, Runtime};
use clorinde::tokio_postgres::NoTls;

pub async fn create_pool() -> Result<Pool, CreatePoolError> {
    let mut cfg = Config::new();
    cfg.user = Some(String::from("postgres"));
    cfg.password = Some(String::from("postgres"));
    cfg.host = Some(String::from("127.0.0.1"));
    cfg.port = Some(5435);
    cfg.dbname = Some(String::from("lapa_clorinde"));
    cfg.create_pool(Some(Runtime::Tokio1), NoTls)
}

pub fn use_pool() -> Result<Pool, ServerFnError> {
    use_context::<Pool>()
        .ok_or("Pool missing.")
        .map_err(|e| ServerFnError::new(e.to_string()))
}
pub async fn use_db() -> Result<Object, ServerFnError> {
    let pool = use_pool()?;
    let client = pool.clone().get().await.unwrap();
    Ok(client)
}
