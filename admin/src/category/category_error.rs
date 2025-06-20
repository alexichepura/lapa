use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CategoryError {
    #[error("Invalid category ID.")]
    InvalidId,
    #[error("Category not found.")]
    NotFound,
    #[error("Server error.")]
    ServerError,
    #[error("Category create error.")]
    CreateError,
    #[error("Category with same slug already exists.")]
    CreateSlugExists,
}
