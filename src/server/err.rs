use http::status::StatusCode;
use leptos::prelude::*;
use leptos_axum::{ResponseOptions, ResponseParts};

pub fn serverr_404() {
    let res_parts = ResponseParts {
        status: Some(StatusCode::NOT_FOUND),
        ..Default::default()
    };
    let res_options_outer = use_context::<ResponseOptions>();
    if let Some(res_options) = res_options_outer {
        res_options.overwrite(res_parts);
    }
}
