use axum::{
    body::Body,
    extract::{Path, State},
    http::{Request, Response, StatusCode, Uri},
    response::{IntoResponse, Response as AxumResponse},
};
use http::{header::CACHE_CONTROL, HeaderValue};
use image_config::ImageConfig;
use leptos::prelude::*;
use tower::ServiceExt;
use tower_http::services::ServeDir;

use crate::err::AppError;
use crate::err::ErrorTemplate;

const MAX_AGE_MONTH: HeaderValue = HeaderValue::from_static("public, max-age=2592000");
// const MAX_AGE_YEAR: HeaderValue = HeaderValue::from_static("public, max-age=31536000");

pub async fn content_image_handler(
    Path(image_name): Path<String>,
    State(image_config): State<ImageConfig>,
    State(pool): State<clorinde::deadpool_postgres::Pool>,
) -> Result<AxumResponse, StatusCode> {
    let db = pool.clone().get().await.unwrap();
    let db_image = clorinde::queries::admin_content_image::read_ext()
        .bind(&db, &image_name)
        .opt()
        .await
        .map_err(|e|{
            tracing::error!("Content image read_ext error={e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;
    let image_name = format!("/{image_name}.{}", db_image);
    let uri = image_name.parse::<Uri>().map_err(|e|{
        tracing::error!("Content image uri parse error={e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    let mut res = get_static_file(uri, &image_config.content_image_upload_path())
        .await.map_err(|e|{
            tracing::error!("Content image file error={}", e.1);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    if res.status() == StatusCode::OK {
        res.headers_mut().insert(CACHE_CONTROL, MAX_AGE_MONTH);
        Ok(res.into_response())
    } else {
        return Err(StatusCode::NOT_FOUND);
    }
}
pub async fn product_image_handler(
    Path(image_name): Path<String>,
    State(image_config): State<ImageConfig>,
    State(pool): State<clorinde::deadpool_postgres::Pool>,
) -> Result<AxumResponse, StatusCode> {
    let db = pool.clone().get().await.unwrap();
    let db_image = clorinde::queries::admin_product_image::read_ext()
        .bind(&db, &image_name)
        .opt()
        .await
        .map_err(|e|{
            tracing::error!("Product image read_ext error={e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;
    let image_name = format!("/{image_name}.{}", db_image);
    let uri = image_name.parse::<Uri>().map_err(|e|{
        tracing::error!("Product image uri parse error={e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    let mut res = get_static_file(uri, &image_config.product_image_upload_path())
        .await.map_err(|e|{
            tracing::error!("Product image file error={}", e.1);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
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
