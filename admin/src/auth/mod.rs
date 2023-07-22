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

type UserSignal = RwSignal<Option<User>>;
pub fn use_user_signal(cx: Scope) -> UserSignal {
    let user = use_context::<UserSignal>(cx).expect("UserSignal");
    return user;
}

#[server(GetUser, "/api")]
pub async fn get_user(cx: Scope) -> Result<Option<User>, ServerFnError> {
    let auth = crate::server::use_server_auth(cx)?;
    Ok(auth.current_user)
}
