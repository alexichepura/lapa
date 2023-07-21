use axum::{
    body::{boxed, Body, BoxBody},
    extract::Extension,
    http::{Request, Response, StatusCode, Uri},
    response::{IntoResponse, Response as AxumResponse},
};
use http::HeaderValue;
use leptos::*;
use std::sync::Arc;

const MAX_AGE_MONTH: HeaderValue = HeaderValue::from_static("public, max-age=2592000");
// const MAX_AGE_YEAR: HeaderValue = HeaderValue::from_static("public, max-age=31536000");

pub async fn file_and_error_handler(
    uri: Uri,
    Extension(options): Extension<Arc<LeptosOptions>>,
    req: Request<Body>,
) -> AxumResponse {
    use crate::err::AppError;
    use crate::err::ErrorTemplate;

    let options = &*options;
    let uri_path = uri.path();
    let (root, uri, is_img): (String, Uri, bool) = if uri_path.starts_with("/img") {
        let uri_path = uri_path.replace("/img", "");
        let uri = uri_path.parse::<Uri>().unwrap();
        ("img".to_string(), uri, true)
    } else {
        (options.site_root.clone(), uri.clone(), false)
    };
    let mut res = get_static_file(uri, &root).await.unwrap();

    if res.status() == StatusCode::OK {
        if is_img {
            res.headers_mut()
                .insert(http::header::CACHE_CONTROL, MAX_AGE_MONTH);
        }
        res.into_response()
    } else {
        let mut errors = Errors::default();
        errors.insert_with_default_key(AppError::NotFound);
        let handler = leptos_axum::render_app_to_stream(
            options.to_owned(),
            move |cx| view! { cx, <ErrorTemplate outside_errors=errors.clone()/> },
        );
        handler(req).await.into_response()
    }
}

async fn get_static_file(uri: Uri, root: &str) -> Result<Response<BoxBody>, (StatusCode, String)> {
    use tower::ServiceExt;
    use tower_http::services::ServeDir;

    let req = Request::builder()
        .uri(uri.clone())
        .body(Body::empty())
        .unwrap();
    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    // This path is relative to the cargo root
    match ServeDir::new(root).oneshot(req).await {
        Ok(res) => Ok(res.map(boxed)),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {err}"),
        )),
    }
}
