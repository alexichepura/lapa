use http::status::StatusCode;
use leptos::{use_context, Scope};
use leptos_axum::ResponseOptions;

pub fn use_response(cx: Scope) -> ResponseOptions {
    use_context::<ResponseOptions>(cx).expect("to have leptos_axum::ResponseOptions provided")
}

pub fn serverr_401(cx: Scope) {
    let response = use_response(cx);
    response.set_status(StatusCode::UNAUTHORIZED);
}

pub fn serverr_404(cx: Scope) {
    let response = use_response(cx);
    response.set_status(StatusCode::NOT_FOUND);
}

pub fn serverr_400(cx: Scope) {
    let response = use_response(cx);
    response.set_status(StatusCode::BAD_REQUEST);
}
