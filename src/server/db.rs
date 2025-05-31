use leptos::prelude::{use_context, ServerFnError};
use clorinde::deadpool_postgres::{Object, Pool};

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
