use axum::{
    body::Body,
    extract::{Path, State},
    http::{Request, Response, StatusCode, Uri},
    response::{IntoResponse, Response as AxumResponse},
};
use http::{header::CACHE_CONTROL, HeaderValue};
use leptos::prelude::*;
use tower::ServiceExt;
use tower_http::services::ServeDir;

use crate::err::AppError;
use crate::err::ErrorTemplate;

use super::MediaConfig;

const MAX_AGE_MONTH: HeaderValue = HeaderValue::from_static("public, max-age=2592000");
// const MAX_AGE_YEAR: HeaderValue = HeaderValue::from_static("public, max-age=31536000");

pub async fn img_handler(Path(img_name): Path<String>, req: Request<Body>) -> AxumResponse {
    let img_name = format!("/{img_name}");
    let uri = img_name.parse::<Uri>().unwrap();
    let mut res = get_static_file(uri, &"img").await.unwrap();
    if res.status() == StatusCode::OK {
        res.headers_mut().insert(CACHE_CONTROL, MAX_AGE_MONTH);
        res.into_response()
    } else {
        not_found_response(req).await
    }
}
pub async fn content_image_handler(
    Path(image_name): Path<String>,
    State(media_config): State<MediaConfig>,
    State(pool): State<clorinde::deadpool_postgres::Pool>,
) -> Result<AxumResponse, StatusCode> {
    let db = pool.clone().get().await.unwrap();
    let db_image = clorinde::queries::admin_content_image::read_ext()
        .bind(&db, &image_name)
        .opt()
        .await
        .inspect_err(|e| tracing::error!("Content image read_ext error={e}"));
    let Ok(db_image) = db_image else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };
    let Some(db_image) = db_image else {
        return Err(StatusCode::NOT_FOUND);
    };

    let image_name = format!("/{image_name}.{}", db_image);
    let uri = image_name.parse::<Uri>().unwrap();
    let mut res = get_static_file(uri, &media_config.image_upload_path)
        .await
        .unwrap();
    if res.status() == StatusCode::OK {
        res.headers_mut().insert(CACHE_CONTROL, MAX_AGE_MONTH);
        Ok(res.into_response())
    } else {
        return Err(StatusCode::NOT_FOUND);
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
        not_found_response(req).await
    }
}

async fn get_static_file(uri: Uri, root: &str) -> Result<Response<Body>, (StatusCode, String)> {
    let req = Request::builder()
        .uri(uri.clone())
        .body(Body::empty())
        .unwrap();
    match ServeDir::new(root).oneshot(req).await {
        Ok(res) => Ok(res.into_response()),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {err}"),
        )),
    }
}

pub async fn not_found_response(req: Request<Body>) -> AxumResponse {
    tracing::warn!("404 not_found_response for: {:?}", req.uri());
    let mut errors = Errors::default();
    errors.insert_with_default_key(AppError::NotFound);
    let handler = leptos_axum::render_app_to_stream(
        move || view! { <ErrorTemplate outside_errors=errors.clone()></ErrorTemplate> },
    );
    handler(req).await.into_response()
}
