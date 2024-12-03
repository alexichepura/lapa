use http::status::StatusCode;
use leptos::prelude::use_context;
use leptos_axum::ResponseOptions;

pub fn use_response() -> ResponseOptions {
    use_context::<ResponseOptions>().expect("to have leptos_axum::ResponseOptions provided")
}

pub fn serverr_401() {
    let response = use_response();
    response.set_status(StatusCode::UNAUTHORIZED);
}

pub fn serverr_404() {
    let response = use_response();
    response.set_status(StatusCode::NOT_FOUND);
}

pub fn serverr_400() {
    let response = use_response();
    response.set_status(StatusCode::BAD_REQUEST);
}
