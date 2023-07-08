mod auth_data;
mod auth_login;
mod auth_logout;
mod auth_signup;
pub use auth_login::Login;
pub use auth_logout::Logout;
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
}

#[server(GetUser, "/api")]
pub async fn get_user(cx: Scope) -> Result<Option<User>, ServerFnError> {
    let auth = ssr_auth(cx)?;
    Ok(auth.current_user)
}

cfg_if::cfg_if! {if #[cfg(feature = "ssr")] {
    use crate::prisma::ArcPrisma;
    use crate::axum_session_prisma::SessionPrismaPool;
    use axum_session_auth::Authentication;
}}

#[cfg(feature = "ssr")]
pub type AuthSession = axum_session_auth::AuthSession<User, String, SessionPrismaPool, ArcPrisma>;

#[cfg(feature = "ssr")]
pub fn ssr_auth(cx: Scope) -> Result<AuthSession, ServerFnError> {
    use_context::<AuthSession>(cx)
        .ok_or("Auth session missing.")
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

#[cfg(feature = "ssr")]
#[async_trait::async_trait]
impl Authentication<User, String, ArcPrisma> for User {
    async fn load_user(userid: String, pool: Option<&ArcPrisma>) -> Result<User, anyhow::Error> {
        let prisma_client = pool.unwrap();

        let db_user = prisma_client
            .user()
            .find_unique(prisma_client::db::user::id::equals(userid))
            .exec()
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?
            .ok_or("User does not exist.")
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        let user = User {
            id: db_user.id,
            username: db_user.username,
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
