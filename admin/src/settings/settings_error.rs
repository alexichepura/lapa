use leptos::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SettingsError {
    #[error("Settings server error.")]
    ServerError,
    #[error("Settings update error.")]
    UpdateError,
    #[error("Settings not found.")]
    NotFound,
}
