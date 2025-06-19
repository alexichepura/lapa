use leptos::{prelude::*};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageUploadError {
    #[error("Image upload server error")]
    ServerError,
    #[error("Image upload deserialization error")]
    Deserialization,
    #[error("Image upload read error.")]
    Read,
    #[error("Image upload format error.")]
    Format,
}


#[server(ContentImageUploadAction, "/api")]
pub async fn content_image_upload(
    img: String,
    alt: String,
    content_id: String,
) -> Result<Result<ImageResult, ImageUploadError>, ServerFnError> {
    use crate::server::{db, serverr_400, use_media_config};
    let db = db::use_db().await?;
    let media_config = use_media_config()?;

    let img_bytes = serde_json::from_str::<Vec<u8>>(&img);
    if let Err(e) = img_bytes {
        tracing::error!("{e:?}");
        serverr_400();
        return Ok(Err(ImageUploadError::Deserialization));
    }
    let img_bytes = img_bytes.unwrap();
    let cursor = std::io::Cursor::new(img_bytes.clone());
    let img_reader = image::ImageReader::new(cursor.clone()).with_guessed_format();

    if let Err(e) = img_reader {
        tracing::error!("{e:?}");
        serverr_400();
        return Ok(Err(ImageUploadError::Read));
    }
    let img_reader = img_reader.unwrap();

    let img_format = img_reader.format();
    if let None = img_format {
        serverr_400();
        return Ok(Err(ImageUploadError::Format));
    }
    let img_format = img_format.unwrap();
    let ext = img_format.extensions_str().first().unwrap();

    let id = cuid2::create_id();
    clorinde::queries::admin_content_image::create()
        .bind(
            &db,
            &id,
            &alt,
            &ext,
            &content_id
        )
        .await
        .map_err(|e| lib::emsg(e, "Content image create"))?;

    let file_path = media_config.content_image_upload_name_ext(&id, &ext.to_string());
    tracing::debug!("upload file_path={file_path}");
    std::fs::write(file_path, img_bytes).map_err(|e| lib::emsg(e, "Content image write"))?;
    Ok(Ok(ImageResult { id }))
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageResult {
    pub id: String,
}
