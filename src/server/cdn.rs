// use super::{ArcPrisma, MediaConfig};
// use crate::server::{serve_file, MAX_AGE_MONTH};
use std::fs::File;
use std::io::BufReader;

use axum::response::IntoResponse;
use axum::{
    extract::{Path, State},
    http::{StatusCode, Uri},
    response::Response as AxumResponse,
};
use content::{CdnImageFormat, CdnImageSize};
use http::HeaderValue;
use http::header::VARY;
use http::{HeaderMap, header::CACHE_CONTROL};
use image::imageops::FilterType;
use image::metadata::Orientation;
use image::{DynamicImage, ImageDecoder, ImageError, ImageFormat, ImageReader};
use image_config::ImageConfig;

use super::{MAX_AGE_MONTH, serve_file};

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

/// Decode an image and bake in its EXIF orientation.
///
/// `image`'s decoder reports [`Orientation::NoTransforms`] for formats (or
/// files) without orientation metadata, so EXIF-less images decode as-is rather
/// than erroring — the previous `kamadak-exif` implementation propagated that
/// "no EXIF" error and rejected such uploads.
fn open_oriented(path: &str) -> Result<DynamicImage, ImageError> {
    let reader = ImageReader::new(BufReader::new(File::open(path)?)).with_guessed_format()?;
    let mut decoder = reader.into_decoder()?;
    // A malformed Exif chunk must not fail the conversion.
    let orientation = decoder.orientation().unwrap_or(Orientation::NoTransforms);
    let mut image = DynamicImage::from_decoder(decoder)?;
    image.apply_orientation(orientation);
    Ok(image)
}

pub async fn content_image_handler(
    Path(img_name): Path<String>,
    State(image_config): State<ImageConfig>,
    State(pool): State<cornucopia::deadpool_postgres::Pool>,
    headers: HeaderMap,
) -> Result<AxumResponse, StatusCode> {
    let (image_name, img_size) = img_name.split_once("_").ok_or(StatusCode::NOT_FOUND)?;
    let img_size = img_size
        .parse::<CdnImageSize>()
        .map_err(|_e| StatusCode::NOT_FOUND)?;

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
    let mut cdn_res = serve_file(&uri, &image_config.content_image_convert_path())
        .await
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;
    if cdn_res.status() == StatusCode::OK {
        cdn_res.headers_mut().insert(CACHE_CONTROL, MAX_AGE_MONTH);
        cdn_res.headers_mut().insert(VARY, VARY_ACCEPT);
        Ok(cdn_res.into_response())
    } else {
        let db = pool.clone().get().await.unwrap();
        let db_image_ext = cornucopia::queries::content_image::read_ext()
            .bind(&db, &image_name)
            .opt()
            .await
            .map_err(|e| {
                tracing::error!("Content image read_ext error={e}");
                StatusCode::INTERNAL_SERVER_ERROR
            })?
            .ok_or(StatusCode::NOT_FOUND)?;

        let img_name = format!("/{image_name}.{}", db_image_ext);
        let path = format!("{}{}", image_config.content_image_upload_path(), img_name);
        tracing::trace!("upload_path={}", path);
        let dynamic_image = open_oriented(&path).map_err(|e| {
            tracing::error!("cdn image error={e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
        let variant = dynamic_image.resize(img_size.to_width(), u32::MAX, FilterType::Lanczos3);
        let cdn_image_path = format!("{}{}", image_config.content_image_convert_path(), uri);
        variant
            .save_with_format(cdn_image_path, cdn_format.to_image_format())
            .map_err(|e| {
                tracing::error!("cdn image save_with_format error={e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        let mut cdn_res = serve_file(&uri, &image_config.content_image_convert_path())
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
    State(image_config): State<ImageConfig>,
    State(pool): State<cornucopia::deadpool_postgres::Pool>,
    headers: HeaderMap,
) -> Result<AxumResponse, StatusCode> {
    let (image_name, img_size) = img_name.split_once("_").ok_or(StatusCode::NOT_FOUND)?;
    let img_size = img_size
        .parse::<CdnImageSize>()
        .map_err(|_e| StatusCode::NOT_FOUND)?;

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
    let mut cdn_res = serve_file(&uri, &image_config.product_image_convert_path())
        .await
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;
    if cdn_res.status() == StatusCode::OK {
        cdn_res.headers_mut().insert(CACHE_CONTROL, MAX_AGE_MONTH);
        cdn_res.headers_mut().insert(VARY, VARY_ACCEPT);
        Ok(cdn_res.into_response())
    } else {
        let db = pool.clone().get().await.unwrap();
        let db_image_ext = cornucopia::queries::product_image::read_ext()
            .bind(&db, &image_name)
            .opt()
            .await
            .map_err(|e| {
                tracing::error!("Product image read_ext error={e}");
                StatusCode::INTERNAL_SERVER_ERROR
            })?
            .ok_or(StatusCode::NOT_FOUND)?;

        let img_name = format!("/{image_name}.{}", db_image_ext);
        let path = format!("{}{}", image_config.product_image_upload_path(), img_name);
        tracing::trace!("upload_path={}", path);
        let dynamic_image = open_oriented(&path).map_err(|e| {
            tracing::error!("cdn image error={e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
        let variant = dynamic_image.resize(img_size.to_width(), u32::MAX, FilterType::Lanczos3);
        let cdn_image_path = format!("{}{}", image_config.product_image_convert_path(), uri);
        variant
            .save_with_format(cdn_image_path, cdn_format.to_image_format())
            .map_err(|e| {
                tracing::error!("cdn image save_with_format error={e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        let mut cdn_res = serve_file(&uri, &image_config.product_image_convert_path())
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

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgb};

    fn jpeg_bytes(width: u32, height: u32) -> Vec<u8> {
        let img =
            DynamicImage::ImageRgb8(ImageBuffer::from_pixel(width, height, Rgb([200, 60, 60])));
        let mut buf = std::io::Cursor::new(Vec::new());
        img.write_to(&mut buf, ImageFormat::Jpeg)
            .expect("encode jpeg");
        buf.into_inner()
    }

    /// Splice an EXIF APP1 segment carrying `Orientation = orientation` in right
    /// after the SOI marker, so `image`'s decoder reads it back.
    fn with_exif_orientation(jpeg: &[u8], orientation: u16) -> Vec<u8> {
        assert_eq!(&jpeg[..2], &[0xFF, 0xD8], "JPEG must start with SOI");

        let mut tiff = Vec::new();
        tiff.extend_from_slice(b"II\x2A\x00"); // little-endian TIFF magic
        tiff.extend_from_slice(&8u32.to_le_bytes()); // IFD0 offset
        tiff.extend_from_slice(&1u16.to_le_bytes()); // one IFD entry
        tiff.extend_from_slice(&0x0112u16.to_le_bytes()); // Orientation tag
        tiff.extend_from_slice(&3u16.to_le_bytes()); // type SHORT
        tiff.extend_from_slice(&1u32.to_le_bytes()); // count
        let mut value = [0u8; 4];
        value[..2].copy_from_slice(&orientation.to_le_bytes());
        tiff.extend_from_slice(&value);
        tiff.extend_from_slice(&0u32.to_le_bytes()); // no next IFD

        let mut payload = Vec::new();
        payload.extend_from_slice(b"Exif\x00\x00");
        payload.extend_from_slice(&tiff);

        let mut out = Vec::new();
        out.extend_from_slice(&jpeg[..2]); // SOI
        out.push(0xFF);
        out.push(0xE1); // APP1
        out.extend_from_slice(&((payload.len() + 2) as u16).to_be_bytes());
        out.extend_from_slice(&payload);
        out.extend_from_slice(&jpeg[2..]);
        out
    }

    fn temp_path(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("lapa-cdn-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("create temp dir");
        dir.join(name)
    }

    #[test]
    fn decodes_image_without_exif() {
        let path = temp_path("no-exif.jpg");
        std::fs::write(&path, jpeg_bytes(16, 8)).expect("write fixture");
        let img = open_oriented(path.to_str().unwrap()).expect("EXIF-less image must decode");
        assert_eq!((img.width(), img.height()), (16, 8));
    }

    #[test]
    fn applies_exif_orientation() {
        let path = temp_path("orientation-6.jpg");
        std::fs::write(&path, with_exif_orientation(&jpeg_bytes(16, 8), 6)).expect("write fixture");
        let img = open_oriented(path.to_str().unwrap()).expect("oriented image must decode");
        // Orientation 6 is a 90° rotation, so the dimensions swap.
        assert_eq!((img.width(), img.height()), (8, 16));
    }
}
