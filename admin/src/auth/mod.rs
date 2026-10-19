mod auth_data;
mod auth_guard;
mod auth_login;
mod auth_logout;
mod auth_signup;
pub use auth_guard::{Forbidden, RequirePermission};
pub use auth_login::Login;
pub use auth_logout::Logout;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// A user role. Stored as text in `"User"."role"` and parsed fail-closed: an
/// unknown value is treated as the least-privileged role.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Admin,
    Editor,
    Viewer,
}

impl Role {
    pub const ALL: [Role; 3] = [Role::Admin, Role::Editor, Role::Viewer];

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "admin" => Some(Self::Admin),
            "editor" => Some(Self::Editor),
            "viewer" => Some(Self::Viewer),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Admin => "admin",
            Self::Editor => "editor",
            Self::Viewer => "viewer",
        }
    }

    pub fn permissions(self) -> &'static [Permission] {
        match self {
            Self::Admin => &[
                Permission::ContentRead,
                Permission::ContentWrite,
                Permission::SettingsManage,
            ],
            Self::Editor => &[Permission::ContentRead, Permission::ContentWrite],
            Self::Viewer => &[Permission::ContentRead],
        }
    }

    pub fn can(self, permission: Permission) -> bool {
        self.permissions().contains(&permission)
    }
}

/// A capability checked by server functions and used to gate UI/routes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Permission {
    /// View the dashboard, lists and pages.
    ContentRead,
    /// Create/update/delete products, posts, categories, content and images.
    ContentWrite,
    /// Read and update site settings.
    SettingsManage,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    /// Whether the account is enabled, derived from `deactivated_at`
    /// (`deactivated_at IS NULL` means enabled). Disabled users cannot
    /// authenticate and their sessions are rejected (see `server::auth`).
    pub active: bool,
    pub role: Role,
}

impl User {
    pub fn can(&self, permission: Permission) -> bool {
        self.role.can(permission)
    }
}

type UserSignal = RwSignal<Option<User>>;
pub fn use_user_signal() -> UserSignal {
    use_context::<UserSignal>().expect("UserSignal")
}

#[server(GetUser, "/api")]
pub async fn get_user() -> Result<Option<User>, ServerFnError> {
    let auth = crate::server::use_auth()?;
    Ok(auth.user.as_ref().map(User::from))
}
