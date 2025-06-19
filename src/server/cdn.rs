// use super::{ArcPrisma, MediaConfig};
// use crate::server::{serve_file, MAX_AGE_MONTH};
use axum::response::IntoResponse;
use axum::{
    extract::{Path, State},
    http::{StatusCode, Uri},
    response::Response as AxumResponse,
};
use content::{CdnImageFormat, CdnImageSize};
use http::header::VARY;
use http::HeaderValue;
use http::{header::CACHE_CONTROL, HeaderMap};
use image::imageops::FilterType;
use image::ImageFormat;

use super::{serve_file, MediaConfig, MAX_AGE_MONTH};

const VARY_ACCEPT: HeaderValue = HeaderValue::from_static("Accept");

trait ToImageFormat {
    fn to_image_format(&self) -> ImageFormat;
}
impl ToImageFormat for CdnImageFormat {
    fn to_image_format(&self) -> ImageFormat {
        match self {
            CdnImageFormat::Avif => ImageFormat::Avif,
            CdnImageFormat::Webp => ImageFormat::WebP,
            CdnImageFormat::Jpeg => ImageFormat::Jpeg,
        }
    }
}

pub async fn content_image_handler(
    Path(img_name): Path<String>,
    State(media_config): State<MediaConfig>,
    State(pool): State<clorinde::deadpool_postgres::Pool>,
    headers: HeaderMap,
) -> Result<AxumResponse, StatusCode> {
    let (image_name, img_size) = img_name.split_once("_").ok_or(StatusCode::NOT_FOUND)?;
    let img_size = CdnImageSize::try_from(img_size).map_err(|_e| StatusCode::NOT_FOUND)?;

    // image/avif,image/webp,image/png,image/svg+xml,image/*;q=0.8,*/*;q=0.5
    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Content_negotiation/List_of_default_Accept_values
    let mut accept_types = headers
        .get(http::header::ACCEPT)
        .and_then(|ct| ct.to_str().ok().map(String::from))
        .unwrap_or_else(|| "*/*".to_string());
    accept_types.retain(|c| !c.is_whitespace());
    accept_types.push(',');
    let cdn_format = CdnImageFormat::from_accept_types(&accept_types);
    let cdn_img_name = format!("/{image_name}_{}.{}", img_size, cdn_format);
    let uri = cdn_img_name.parse::<Uri>().map_err(|e| {
        tracing::error!("cdn uri parse error={e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    let mut cdn_res = serve_file(&uri, &media_config.content_image_convert_path())
        .await
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;
    if cdn_res.status() == StatusCode::OK {
        cdn_res.headers_mut().insert(CACHE_CONTROL, MAX_AGE_MONTH);
        cdn_res.headers_mut().insert(VARY, VARY_ACCEPT);
        Ok(cdn_res.into_response())
    } else {
        let db = pool.clone().get().await.unwrap();
        let db_image_ext = clorinde::queries::content_image::read_ext()
            .bind(&db, &image_name)
            .opt()
            .await
            .map_err(|e|{
                tracing::error!("Content image read_ext error={e}");
                StatusCode::INTERNAL_SERVER_ERROR
            })?
            .ok_or(StatusCode::NOT_FOUND)?;

        let img_name = format!("/{image_name}.{}", db_image_ext);
        let path = format!("{}{}", &media_config.content_image_upload_path(), img_name);
        tracing::trace!("upload_path={}", path);
        let dynamic_image = image::open(&path).map_err(|e| {
            tracing::error!("cdn image error={e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
        let variant = dynamic_image.resize(img_size.to_width(), u32::MAX, FilterType::Lanczos3);
        let cdn_image_path = format!("{}{}", &media_config.content_image_convert_path(), uri);
        variant
            .save_with_format(cdn_image_path, cdn_format.to_image_format())
            .map_err(|e| {
                tracing::error!("cdn image save_with_format error={e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        let mut cdn_res = serve_file(&uri, &media_config.content_image_convert_path())
            .await
            .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;
        if cdn_res.status() == StatusCode::OK {
            cdn_res.headers_mut().insert(CACHE_CONTROL, MAX_AGE_MONTH);
            cdn_res.headers_mut().insert(VARY, VARY_ACCEPT);
            Ok(cdn_res.into_response())
        } else {
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
pub async fn product_image_handler(
    Path(img_name): Path<String>,
    State(media_config): State<MediaConfig>,
    State(pool): State<clorinde::deadpool_postgres::Pool>,
    headers: HeaderMap,
) -> Result<AxumResponse, StatusCode> {
    let (image_name, img_size) = img_name.split_once("_").ok_or(StatusCode::NOT_FOUND)?;
    let img_size = CdnImageSize::try_from(img_size).map_err(|_e| StatusCode::NOT_FOUND)?;

    // image/avif,image/webp,image/png,image/svg+xml,image/*;q=0.8,*/*;q=0.5
    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Content_negotiation/List_of_default_Accept_values
    let mut accept_types = headers
        .get(http::header::ACCEPT)
        .and_then(|ct| ct.to_str().ok().map(String::from))
        .unwrap_or_else(|| "*/*".to_string());
    accept_types.retain(|c| !c.is_whitespace());
    accept_types.push(',');
    let cdn_format = CdnImageFormat::from_accept_types(&accept_types);
    let cdn_img_name = format!("/{image_name}_{}.{}", img_size, cdn_format);
    let uri = cdn_img_name.parse::<Uri>().map_err(|e| {
        tracing::error!("cdn uri parse error={e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    let mut cdn_res = serve_file(&uri, &media_config.product_image_convert_path())
        .await
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;
    if cdn_res.status() == StatusCode::OK {
        cdn_res.headers_mut().insert(CACHE_CONTROL, MAX_AGE_MONTH);
        cdn_res.headers_mut().insert(VARY, VARY_ACCEPT);
        Ok(cdn_res.into_response())
    } else {
        let db = pool.clone().get().await.unwrap();
        let db_image_ext = clorinde::queries::product_image::read_ext()
            .bind(&db, &image_name)
            .opt()
            .await
            .map_err(|e|{
                tracing::error!("Product image read_ext error={e}");
                StatusCode::INTERNAL_SERVER_ERROR
            })?
            .ok_or(StatusCode::NOT_FOUND)?;

        let img_name = format!("/{image_name}.{}", db_image_ext);
        let path = format!("{}{}", &media_config.product_image_upload_path(), img_name);
        tracing::trace!("upload_path={}", path);
        let dynamic_image = image::open(&path).map_err(|e| {
            tracing::error!("cdn image error={e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
        let variant = dynamic_image.resize(img_size.to_width(), u32::MAX, FilterType::Lanczos3);
        let cdn_image_path = format!("{}{}", &media_config.product_image_convert_path(), uri);
        variant
            .save_with_format(cdn_image_path, cdn_format.to_image_format())
            .map_err(|e| {
                tracing::error!("cdn image save_with_format error={e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        let mut cdn_res = serve_file(&uri, &media_config.product_image_convert_path())
            .await
            .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;
        if cdn_res.status() == StatusCode::OK {
            cdn_res.headers_mut().insert(CACHE_CONTROL, MAX_AGE_MONTH);
            cdn_res.headers_mut().insert(VARY, VARY_ACCEPT);
            Ok(cdn_res.into_response())
        } else {
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
