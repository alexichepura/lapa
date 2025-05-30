use axum_session_auth::Authentication;
use axum_session_auth::{AuthConfig, AuthSessionLayer};
use leptos::prelude::{use_context, ServerFnError};

use super::SessionPool;
use crate::auth::User;
use clorinde::deadpool_postgres::Pool;

type PoolAuthSessionLayer = AuthSessionLayer<User, String, SessionPool, Pool>;

pub fn auth_session_layer(pool: &Pool) -> PoolAuthSessionLayer {
    let auth_config = AuthConfig::<String>::default();
    let layer = PoolAuthSessionLayer::new(Some(pool.clone())).with_config(auth_config);
    layer
}

pub type AuthSession = axum_session_auth::AuthSession<User, String, SessionPool, Pool>;

pub fn use_auth() -> Result<AuthSession, ServerFnError> {
    use_context::<AuthSession>()
        .ok_or("Auth session missing.")
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[async_trait::async_trait]
impl Authentication<User, String, Pool> for User {
    async fn load_user(userid: String, pool: Option<&Pool>) -> Result<User, anyhow::Error> {
        let db = crate::server::db::use_db().await.unwrap();
        let username = clorinde::queries::user::user_find_by_id()
            .bind(&db, &userid).opt()
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?
            .ok_or("User does not exist.")
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;          

        let user = User {
            id: userid,
            username,
        };
        Ok(user)
    }

    fn is_authenticated(&self) -> bool {
        true
    }

    fn is_active(&self) -> bool {
        true
    }

    fn is_anonymous(&self) -> bool {
        false
    }
}
