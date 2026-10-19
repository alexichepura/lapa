use axum_session_auth::Authentication;
use axum_session_auth::{AuthConfig, AuthSessionLayer};
use leptos::prelude::{ServerFnError, use_context};

use super::SessionPool;
use crate::auth::User;
use cornucopia::deadpool_postgres::Pool;

type PoolAuthSessionLayer = AuthSessionLayer<User, String, SessionPool, Pool>;

pub fn auth_session_layer(pool: &Pool) -> PoolAuthSessionLayer {
    // Disable the in-memory user cache: `Authentication::load_user` must run on
    // every request so that disabling an account takes effect on its very next
    // request (fails closed) without bulk session revocation. The default cache
    // would keep serving the stale, active `User` until it expired.
    let auth_config = AuthConfig::<String>::default().set_cache(false);

    PoolAuthSessionLayer::new(Some(pool.clone())).with_config(auth_config)
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
        let Some(pool) = pool else {
            return Err(anyhow::anyhow!("no pool"));
        };
        let db = pool.clone().get().await.unwrap();
        let user = cornucopia::queries::user::user_find_by_id()
            .bind(&db, &userid)
            .opt()
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?
            .ok_or("User does not exist.")
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        // Disabled accounts fail closed: returning `Err` here keeps
        // `AuthSession::current_user` at `None`, so an already-issued session
        // stops working on its next request (no bulk session revocation needed)
        // and every guard/predicate below observes a logged-out user.
        if user.deactivated_at.is_some() {
            return Err(anyhow::anyhow!("user is inactive"));
        }

        let user = User {
            id: userid,
            username: user.username,
            // `active` is derived: `deactivated_at IS NULL` is the enabled state.
            active: user.deactivated_at.is_none(),
        };
        Ok(user)
    }

    /// A loaded `User` is only ever constructed for an authenticated session, so
    /// `is_authenticated` is `true` by construction. `is_active` reflects the
    /// stored flag as defense-in-depth for any direct `User` construction.
    fn is_authenticated(&self) -> bool {
        true
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn is_anonymous(&self) -> bool {
        false
    }
}
