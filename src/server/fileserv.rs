use axum::{
    body::{boxed, Body, BoxBody},
    extract::{Path, State},
    http::{Request, Response, StatusCode, Uri},
    response::{IntoResponse, Response as AxumResponse},
};
use http::{header::CACHE_CONTROL, HeaderValue};
use leptos::*;
use tower::ServiceExt;
use tower_http::services::ServeDir;

use crate::err::AppError;
use crate::err::ErrorTemplate;

const MAX_AGE_MONTH: HeaderValue = HeaderValue::from_static("public, max-age=2592000");
// const MAX_AGE_YEAR: HeaderValue = HeaderValue::from_static("public, max-age=31536000");

pub async fn img_handler(
    Path(img_name): Path<String>,
    State(options): State<LeptosOptions>,
    req: Request<Body>,
) -> AxumResponse {
    let img_name = format!("/{img_name}");
    let uri = img_name.parse::<Uri>().unwrap();
    let mut res = get_static_file(uri, &"img").await.unwrap();
    if res.status() == StatusCode::OK {
        res.headers_mut().insert(CACHE_CONTROL, MAX_AGE_MONTH);
        res.into_response()
    } else {
        not_found_response(req, &options).await
    }
}

pub async fn file_and_error_handler(
    uri: Uri,
    State(options): State<LeptosOptions>,
    req: Request<Body>,
) -> AxumResponse {
    let res = get_static_file(uri, &options.site_root).await.unwrap();
    if res.status() == StatusCode::OK {
        res.into_response()
    } else {
        not_found_response(req, &options).await
    }
}

async fn get_static_file(uri: Uri, root: &str) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let req = Request::builder()
        .uri(uri.clone())
        .body(Body::empty())
        .unwrap();
    match ServeDir::new(root).oneshot(req).await {
        Ok(res) => Ok(res.map(boxed)),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {err}"),
        )),
    }
}

pub async fn not_found_response(req: Request<Body>, options: &LeptosOptions) -> AxumResponse {
    let mut errors = Errors::default();
    errors.insert_with_default_key(AppError::NotFound);
    let handler = leptos_axum::render_app_to_stream(
        options.to_owned(),
        move || view! { <ErrorTemplate outside_errors=errors.clone()/> },
    );
    handler(req).await.into_response()
}
