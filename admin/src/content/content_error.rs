use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContentError {
    #[error("Content invalid ID.")]
    InvalidId,
    #[error("Content not found.")]
    NotFound,
    #[error("Content server error.")]
    Server,
    #[error("Content create error.")]
    Create,
    #[error("Content with same slug already exists.")]
    CreateSlugExists,
}
