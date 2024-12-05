use leptos::prelude::*;

pub fn emsg(e: impl std::error::Error, msg: impl ToString) -> ServerFnError {
    tracing::error!("{} error: {e}", msg.to_string());
    ServerFnError::new(msg)
}
