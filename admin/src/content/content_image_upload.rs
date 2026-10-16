use leptos::{
    prelude::*,
    server_fn::codec::{MultipartData, MultipartFormData},
};
use serde::{Deserialize, Serialize};

use crate::upload::ImageUploadError;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImageResult {
    pub id: String,
}

#[server(ContentImageUploadAction, "/api", input = MultipartFormData)]
pub async fn content_image_upload(
    data: MultipartData,
) -> Result<Result<ImageResult, ImageUploadError>, ServerFnError> {
    use crate::server::{db, serverr_400, use_image_config};
    use crate::upload::MAX_IMAGE_BYTES;
    let db = db::use_db().await?;
    let media_config = use_image_config()?;

    let mut data = data.into_inner().ok_or_else(|| {
        serverr_400();
        ImageUploadError::Read
    })?;

    let mut img_bytes = Vec::new();
    let mut alt = String::new();
    let mut content_id = String::new();
    while let Ok(Some(mut field)) = data.next_field().await {
        match field.name().unwrap_or_default() {
            "img" => {
                while let Ok(Some(chunk)) = field.chunk().await {
                    if img_bytes.len() + chunk.len() > MAX_IMAGE_BYTES {
                        serverr_400();
                        return Ok(Err(ImageUploadError::TooLarge));
                    }
                    img_bytes.extend_from_slice(&chunk);
                }
            }
            "alt" => alt = field.text().await.unwrap_or_default(),
            "content_id" => content_id = field.text().await.unwrap_or_default(),
            _ => {}
        }
    }

    let cursor = std::io::Cursor::new(&img_bytes);
    let img_reader = image::ImageReader::new(cursor)
        .with_guessed_format()
        .map_err(|e| {
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
    cornucopia::queries::admin_content_image::create()
        .bind(&db, &id, &alt, &ext, &content_id)
        .await
        .map_err(|e| lib::emsg(e, "Content image create"))?;

    let file_path = media_config.content_image_upload_name_ext(&id, &ext.to_string());
    std::fs::write(file_path, img_bytes).map_err(|e| lib::emsg(e, "Content image write"))?;

    Ok(Ok(ImageResult { id }))
}
