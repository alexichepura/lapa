use axum_session_auth::Authentication;
use axum_session_auth::{AuthConfig, AuthSessionLayer};
use leptos::prelude::{use_context, ServerFnError};

use super::SessionPrismaPool;
use crate::auth::User;
use crate::server::ArcPrisma;

type PrismaAuthSessionLayer = AuthSessionLayer<User, String, SessionPrismaPool, ArcPrisma>;

pub fn auth_session_layer(prisma: &ArcPrisma) -> PrismaAuthSessionLayer {
    let auth_config = AuthConfig::<String>::default();
    let layer = PrismaAuthSessionLayer::new(Some(prisma.clone())).with_config(auth_config);
    layer
}

pub type AuthSession = axum_session_auth::AuthSession<User, String, SessionPrismaPool, ArcPrisma>;

pub fn use_auth() -> Result<AuthSession, ServerFnError> {
    use_context::<AuthSession>()
        .ok_or("Auth session missing.")
        .map_err(|e| ServerFnError::new(e.to_string()))
}

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
