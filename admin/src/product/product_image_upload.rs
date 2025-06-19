use leptos::{either::Either, prelude::*};
use serde::{Deserialize, Serialize};

use crate::{form::FormFooter, upload::{ImageUploadError, InputImage}};

#[component]
pub fn ImageUpload(product_id: String, image_upload: ServerAction<ImageUpload>) -> impl IntoView {
    let pending = image_upload.pending();
    let (_file_name, set_file_name) = signal(None::<String>);
    let (save_byte_vec, set_save_byte_vec) = signal(None::<Vec<u8>>);
    let (_save_file, set_save_file) = signal(None::<String>);
    let (obj_url, set_obj_url) = signal(None::<String>);
    let img_value = move || {
        Some(
            serde_json::to_string(
                    &save_byte_vec().unwrap_or_default().to_vec(),
                )
                .unwrap(),
        )
    };
    view! {
        <fieldset prop:disabled=move || pending()>
            <legend>Image upload</legend>
            <div class="Grid-fluid-2">
                <div>
                    <label>
                        <div>Select image</div>
                        <InputImage set_file_name set_save_file set_obj_url set_save_byte_vec />
                    </label>
                    <ActionForm action=image_upload>
                        <input type="hidden" name="post_id" value=product_id />
                        <input type="hidden" name="img" value=img_value />
                        <label>
                            <span>Alt</span>
                            <input name="alt" />
                        </label>
                        <FormFooter action=image_upload submit_text="Upload image" />
                    </ActionForm>
                </div>
                <ImageUploadPreview obj_url />
            </div>
        </fieldset>
    }
}

#[component]
pub fn ImageUploadPreview(obj_url: ReadSignal<Option<String>>) -> impl IntoView {
    let view = move || match obj_url.get() {
        Some(url) => Either::Left(view! { <img src=url /> }),
        None => Either::Right(view! { <p>Upload preview</p> }),
    };
    view! { <div class="ImageUploadPreview">{view}</div> }
}

type ImageUploadResult = Result<ImageResult, ImageUploadError>;

#[server(ImageUpload, "/api")]
pub async fn upload_img(
    img: String,
    alt: String,
    post_id: String,
) -> Result<ImageUploadResult, ServerFnError> {
    use crate::server::{db, serverr_400, use_media_config};
    let db = db::use_db().await?;
    let media_config = use_media_config()?;
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
    let _created = clorinde::queries::admin_product_image::create()
        .bind(&db, &id, &alt, ext, &post_id)
        .await
        .map_err(|e| lib::emsg(e, "Product image create"))?;

    let file_path = media_config.product_image_upload_name_ext(&id, &ext.to_string());
    std::fs::write(file_path, img_bytes).map_err(|e| lib::emsg(e, "Product image write"))?;
    // let img_decoded = img_reader.decode().unwrap();
    // let buffered_read = std::io::BufReader::new(cursor);
    // crate::image::create_image_variants_from_buf(
    //     buffered_read,
    //     img_decoded,
    //     &convert_settings,
    //     &id,
    // )
    // .map_err(|e| lib::emsg(e, "Image create variants"))?;
    Ok(Ok(ImageResult { id }))
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageResult {
    id: String,
}
