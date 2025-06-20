use leptos::{prelude::*};
use serde::{Deserialize, Serialize};

use crate::upload::ImageUploadError;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImageResult {
    pub id: String,
}

#[server(ContentImageUploadAction, "/api")]
pub async fn content_image_upload(
    img: String,
    alt: String,
    content_id: String,
) -> Result<Result<ImageResult, ImageUploadError>, ServerFnError> {
    use crate::server::{db, serverr_400, use_image_config};
    let db = db::use_db().await?;
    let media_config = use_image_config()?;
    let img_bytes = serde_json::from_str::<Vec<u8>>(&img).map_err(|e| {
        tracing::error!("{e:?}");
        serverr_400();
        ImageUploadError::Deserialization
    })?;
    let cursor = std::io::Cursor::new(img_bytes.clone());
    let img_reader = image::ImageReader::new(cursor.clone()).with_guessed_format().map_err(|e| {
        tracing::error!("{e:?}");
        serverr_400();
        ImageUploadError::Read
    })?;
    let img_format = img_reader.format().ok_or_else(|| {
        serverr_400();
        ImageUploadError::Format
    })?;
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
    std::fs::write(file_path, img_bytes).map_err(|e| lib::emsg(e, "Content image write"))?;

    Ok(Ok(ImageResult { id }))
}

