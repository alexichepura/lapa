use std::collections::HashSet;

use axum_login::{AuthnBackend, AuthzBackend, UserId};
use cornucopia::deadpool_postgres::Pool;
use cornucopia::queries;
use leptos::prelude::{ServerFnError, use_context};

use crate::auth::{Permission, Role, User};

/// The backend user, as loaded from the database. Kept separate from the
/// serializable [`User`] so the password hash never reaches the client.
#[derive(Clone, Debug)]
pub struct AuthUser {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub role: Role,
}

impl axum_login::AuthUser for AuthUser {
    type Id = String;

    fn id(&self) -> Self::Id {
        self.id.clone()
    }

    /// Binds the session to the current password hash: changing the password
    /// invalidates existing sessions. Deactivation is handled by `get_user`
    /// returning `None`.
    fn session_auth_hash(&self) -> &[u8] {
        self.password_hash.as_bytes()
    }
}

impl From<&AuthUser> for User {
    fn from(user: &AuthUser) -> Self {
        User {
            id: user.id.clone(),
            username: user.username.clone(),
            active: true,
            role: user.role,
        }
    }
}

/// The `axum-login` backend: authenticates credentials and loads users by id.
#[derive(Clone)]
pub struct Backend {
    pool: Pool,
}

impl Backend {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

/// Backend failures are infrastructure errors (DB/pool/hashing), never
/// "wrong credentials" or "deactivated" — those are `Ok(None)`. `axum-login`
/// maps a backend error during session load to a 500, so a transient DB
/// failure does not silently log everyone out.
#[derive(Debug, thiserror::Error)]
pub enum BackendError {
    #[error("connection pool error: {0}")]
    Pool(String),
    #[error("database error: {0}")]
    Db(#[from] deadpool_postgres::tokio_postgres::Error),
    #[error("password verification error: {0}")]
    Bcrypt(#[from] bcrypt::BcryptError),
}

/// Parse the stored role, failing closed to `viewer` on an unknown value.
fn parse_role(stored: &str) -> Role {
    Role::parse(stored).unwrap_or_else(|| {
        tracing::warn!(role = stored, "unknown user role; treating as viewer");
        Role::Viewer
    })
}

// `AuthnBackend` declares its methods as `fn ... -> impl Future` (late-bound
// `&self`), so implementing them as `async fn` (early-bound lifetimes) fails to
// compile; the RPITIT form is required here.
#[allow(clippy::manual_async_fn)]
impl AuthnBackend for Backend {
    type User = AuthUser;
    type Credentials = Credentials;
    type Error = BackendError;

    fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> impl std::future::Future<Output = Result<Option<AuthUser>, BackendError>> + Send {
        async move {
            let db = self
                .pool
                .get()
                .await
                .map_err(|e| BackendError::Pool(e.to_string()))?;
            let found = queries::user::user_get_auth_by_username()
                .bind(&db, &creds.username)
                .opt()
                .await?;
            let Some(user) = found else {
                return Ok(None);
            };
            // Disabled accounts cannot authenticate; fail closed with the same
            // `None` as a bad password.
            if user.deactivated_at.is_some() {
                return Ok(None);
            }
            if !bcrypt::verify(creds.password, &user.password)? {
                return Ok(None);
            }
            Ok(Some(AuthUser {
                id: user.id,
                username: creds.username,
                password_hash: user.password,
                role: parse_role(&user.role),
            }))
        }
    }

    fn get_user(
        &self,
        user_id: &UserId<Self>,
    ) -> impl std::future::Future<Output = Result<Option<AuthUser>, BackendError>> + Send {
        let user_id = user_id.clone();
        async move {
            let db = self
                .pool
                .get()
                .await
                .map_err(|e| BackendError::Pool(e.to_string()))?;
            let found = queries::user::user_find_by_id()
                .bind(&db, &user_id)
                .opt()
                .await?;
            let Some(user) = found else {
                return Ok(None);
            };
            // A disabled account is treated as logged out; the session stops
            // working on its next request.
            if user.deactivated_at.is_some() {
                return Ok(None);
            }
            Ok(Some(AuthUser {
                id: user_id,
                username: user.username,
                password_hash: user.password,
                role: parse_role(&user.role),
            }))
        }
    }
}

// Same rationale as `AuthnBackend`: the trait uses `fn ... -> impl Future`.
#[allow(clippy::manual_async_fn)]
impl AuthzBackend for Backend {
    type Permission = Permission;

    fn get_user_permissions(
        &self,
        user: &AuthUser,
    ) -> impl std::future::Future<Output = Result<HashSet<Permission>, BackendError>> + Send {
        let permissions: HashSet<Permission> = user.role.permissions().iter().copied().collect();
        async move { Ok(permissions) }
    }
}

/// Require the current session user to hold `permission`.
///
/// Server functions call this before mutating; UI and route guards are
/// convenience only, this is the security boundary.
pub async fn require_permission(permission: Permission) -> Result<(), ServerFnError> {
    let auth = use_auth()?;
    let Some(user) = auth.user.as_ref() else {
        return Err(ServerFnError::new("Not authenticated."));
    };
    match auth.backend.has_perm(user, permission).await {
        Ok(true) => Ok(()),
        Ok(false) => Err(ServerFnError::new(
            "Forbidden: your role does not allow this.",
        )),
        Err(e) => Err(ServerFnError::new(e.to_string())),
    }
}

pub type AuthSession = axum_login::AuthSession<Backend>;

pub fn use_auth() -> Result<AuthSession, ServerFnError> {
    use_context::<AuthSession>()
        .ok_or("Auth session missing.")
        .map_err(|e| ServerFnError::new(e.to_string()))
}
