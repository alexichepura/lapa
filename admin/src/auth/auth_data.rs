use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthError {
    #[error("Login-password pair not found")]
    NoMatch,
    #[error("Server error.")]
    ServerError,
}
