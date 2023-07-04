use leptos::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageError {
    #[error("Image invalid ID.")]
    InvalidId,
    #[error("Image not found.")]
    NotFound,
    #[error("Image server error.")]
    ServerError,
}
