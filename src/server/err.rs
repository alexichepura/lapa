use http::status::StatusCode;
use leptos::*;
use leptos_axum::{ResponseOptions, ResponseParts};

pub fn serverr_404(cx: Scope) {
    let res_parts = ResponseParts {
        status: Some(StatusCode::NOT_FOUND),
        ..Default::default()
    };
    let res_options_outer = use_context::<ResponseOptions>(cx);
    if let Some(res_options) = res_options_outer {
        res_options.overwrite(res_parts);
    }
}
