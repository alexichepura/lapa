use leptos::{
    either::Either,
    prelude::*,
    server_fn::codec::{MultipartData, MultipartFormData},
};
use serde::{Deserialize, Serialize};

use crate::{
    form::UploadFooter,
    upload::{ImageUploadError, InputImage},
};

stylance::import_crate_style!(
    #[allow(dead_code)]
    grid,
    "src/grid.module.css"
);
stylance::import_crate_style!(
    #[allow(dead_code)]
    images,
    "src/images.module.css"
);

#[component]
pub fn ImageUpload(product_id: String, version: RwSignal<usize>) -> impl IntoView {
    let pending = RwSignal::new(false);
    let value = RwSignal::<Option<Result<ImageUploadResult, ServerFnError>>>::new(None);
    let (obj_url, set_obj_url) = signal(None::<String>);
    #[cfg(not(feature = "hydrate"))]
    let _ = version;

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        #[cfg(feature = "hydrate")]
        {
            use wasm_bindgen::JsCast;
            ev.prevent_default();
            let target = ev
                .target()
                .unwrap()
                .unchecked_into::<web_sys::HtmlFormElement>();
            let form_data = web_sys::FormData::new_with_form(&target).unwrap();
            pending.set(true);
            leptos::task::spawn_local(async move {
                let result = upload_img(form_data.into()).await;
                if result.as_ref().is_ok_and(|r| r.is_ok()) {
                    version.update(|v| *v += 1);
                }
                value.set(Some(result));
                pending.set(false);
            });
        }
        #[cfg(not(feature = "hydrate"))]
        let _ = ev;
    };

    view! {
        <fieldset prop:disabled=move || pending.get()>
            <legend>Image upload</legend>
            <div class=grid::gridFluid2>
                <div>
                    <form on:submit=on_submit>
                        <label>
                            <div>Select image</div>
                            <InputImage set_obj_url />
                        </label>
                        <input type="hidden" name="post_id" value=product_id />
                        <label>
                            <span>Alt</span>
                            <input name="alt" />
                        </label>
                        <UploadFooter
                            pending=Signal::from(pending)
                            value=Signal::from(value)
                            submit_text="Upload image"
                        />
                    </form>
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
    view! { <div class=images::imageUploadPreview>{view}</div> }
}

type ImageUploadResult = Result<ImageResult, ImageUploadError>;

#[server(ImageUpload, "/api", input = MultipartFormData)]
pub async fn upload_img(data: MultipartData) -> Result<ImageUploadResult, ServerFnError> {
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
    let mut post_id = String::new();
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
            "post_id" => post_id = field.text().await.unwrap_or_default(),
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
    // New uploads land last: `order` is the current image count, matching the
    // 0-based indices assigned by the reorder-on-save path.
    let order = cornucopia::queries::admin_product_image::count_by_product()
        .bind(&db, &post_id)
        .one()
        .await
        .map_err(|e| lib::emsg(e, "Product image count"))?;
    let _created = cornucopia::queries::admin_product_image::create()
        .bind(&db, &id, &alt, ext, &(order as i32), &post_id)
        .await
        .map_err(|e| lib::emsg(e, "Product image create"))?;

    let file_path = media_config.product_image_upload_name_ext(&id, ext.as_ref());
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
