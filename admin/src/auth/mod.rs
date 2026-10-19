mod auth_data;
mod auth_login;
mod auth_logout;
mod auth_signup;
pub use auth_login::Login;
pub use auth_logout::Logout;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    /// Whether the account is enabled. Disabled users cannot authenticate and
    /// their existing sessions fail closed (see `server::auth::load_user`).
    pub active: bool,
}

type UserSignal = RwSignal<Option<User>>;
pub fn use_user_signal() -> UserSignal {
    use_context::<UserSignal>().expect("UserSignal")
}

#[server(GetUser, "/api")]
pub async fn get_user() -> Result<Option<User>, ServerFnError> {
    let auth = crate::server::use_auth()?;
    Ok(auth.current_user)
}
