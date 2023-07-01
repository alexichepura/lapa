// use leptos::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FormError {
    #[error("Form error.")]
    FormError,
}
