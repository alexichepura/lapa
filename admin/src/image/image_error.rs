use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageLoadError {
    #[error("Image not found.")]
    NotFound,
    #[error("Image server error.")]
    ServerError,
}

#[derive(Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageUploadError {
    #[error("Image upload server error")]
    ServerError,
    #[error("Image upload deserialization error")]
    Deserialization,
    #[error("Image upload read error.")]
    Read,
    #[error("Image upload format error.")]
    Format,
}
