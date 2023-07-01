use leptos::*;
use leptos_router::ActionForm;
use serde::{Deserialize, Serialize};

use crate::{
    upload::InputImage,
    util::{AlertDanger, AlertSuccess},
};

#[component]
pub fn ImageUpload(cx: Scope, post_id: String) -> impl IntoView {
    let upload_img = create_server_action::<UploadImg>(cx);
    let value = upload_img.value();
    let pending = upload_img.pending();
    let (file_name, set_file_name) = create_signal(cx, None::<String>);
    let (save_byte_vec, set_save_byte_vec) = create_signal(cx, None::<Vec<u8>>);
    let (_save_file, set_save_file) = create_signal(cx, None::<String>);
    let (obj_url, set_obj_url) = create_signal(cx, None::<String>);
    view! { cx,
        <fieldset disabled=move || pending()>
            <legend>"Image upload"</legend>
            <div>
                <div>{move || file_name.get()}</div>
                <img class="upload-preview" src=obj_url/>
            </div>
            <label>
                <div>"Select image"</div>
                <InputImage set_file_name set_save_file set_obj_url set_save_byte_vec/>
            </label>
            <ActionForm action=upload_img>
                <input type="hidden" name="post_id" value=post_id/>
                <label>
                    <span>"Alt"</span>
                    <input name="alt"/>
                </label>
                <input
                    type="hidden"
                    name="img"
                    value=move || {
                        Some(
                            serde_json::to_string(&save_byte_vec().unwrap_or_default().to_vec())
                                .unwrap(),
                        )
                    }
                />
                <input type="submit" value="Upload"/>
                <Suspense fallback=|| ()>
                    {move || match value() {
                        None => view! { cx, "" }.into_view(cx),
                        Some(v) => {
                            let post_result = v
                                .map_err(|_| ImageUploadError::ServerError)
                                .flatten();
                            match post_result {
                                Ok(_) => view! { cx, <AlertSuccess/> }.into_view(cx),
                                Err(e) => {
                                    view! { cx, <AlertDanger text=e.to_string()/> }.into_view(cx)
                                }
                            }
                        }
                    }}
                </Suspense>
            </ActionForm>
        </fieldset>
    }
}

#[server(UploadImg, "/api")]
pub async fn upload_img(
    cx: Scope,
    img: String,
    post_id: String,
) -> Result<Result<ImageResult, ImageUploadError>, ServerFnError> {
    let img_bytes = serde_json::from_str::<Vec<u8>>(&img);
    if let Err(e) = img_bytes {
        dbg!(e);
        crate::err::serverr_400(cx);
        return Ok(Err(ImageUploadError::Deserialization));
    }
    let img_bytes = img_bytes.unwrap();

    use prisma_client::db;
    let prisma_client = crate::prisma::use_prisma(cx)?;

    let settings = prisma_client
        .settings()
        .find_first(vec![])
        .exec()
        .await
        .map_err(|e| {
            dbg!(e);
            ServerFnError::ServerError("Server error".to_string())
        })?
        .unwrap();

    let hero_size = ImageSize {
        width: settings.hero_width.try_into().unwrap(),
        height: settings.hero_height.try_into().unwrap(),
    };
    let thumb_size = ImageSize {
        width: settings.thumb_width.try_into().unwrap(),
        height: settings.thumb_height.try_into().unwrap(),
    };

    let image_upload_data = prisma_client
        .image()
        .create(
            "".to_string(),
            "".to_string(),
            db::post::UniqueWhereParam::IdEquals(post_id),
            vec![],
        )
        .exec()
        .await
        .map_err(|e| {
            dbg!(e);
            ServerFnError::ServerError("Server error".to_string())
        })?;

    let img_reader =
        image::io::Reader::new(std::io::Cursor::new(img_bytes.clone())).with_guessed_format();

    if let Err(e) = img_reader {
        dbg!(e);
        crate::err::serverr_400(cx);
        return Ok(Err(ImageUploadError::Read));
    }
    let img_reader = img_reader.unwrap();

    let img_format = img_reader.format();
    if let None = img_format {
        crate::err::serverr_400(cx);
        return Ok(Err(ImageUploadError::Format));
    }
    let img_format = img_format.unwrap();
    let format_string = format!("{:?}", img_format);
    let ext = img_format.extensions_str().first().unwrap();
    let id = image_upload_data.id;
    let name = format!("{id}.{ext}");
    let upload_path = "upload";
    let file_path = format!("{upload_path}/{name}");
    std::fs::write(file_path.clone(), img_bytes).map_err(|e| {
        dbg!(e);
        ServerFnError::ServerError("Server error".to_string())
    })?;

    // dbg!(img_format);
    let img_decoded = img_reader.decode().unwrap();
    let height = img_decoded.height();
    let width = img_decoded.width();

    let img_path = "img";

    let hero = img_decoded.resize_to_fill(
        hero_size.width,
        hero_size.height,
        image::imageops::FilterType::Lanczos3,
    );
    let hero_path = format!("{img_path}/{id}-l.webp");
    hero.save_with_format(hero_path, image::ImageFormat::WebP)
        .map_err(|e| {
            dbg!(e);
            ServerFnError::ServerError("Server error".to_string())
        })?;

    let thumb = img_decoded.resize_to_fill(
        thumb_size.width,
        thumb_size.height,
        image::imageops::FilterType::Lanczos3,
    );
    let thumb_path = format!("{img_path}/{id}-s.webp");
    thumb
        .save_with_format(thumb_path, image::ImageFormat::WebP)
        .map_err(|e| {
            dbg!(e);
            ServerFnError::ServerError("Server error".to_string())
        })?;

    Ok(Ok(ImageResult {
        format: format_string,
        height,
        width,
    }))
}

use thiserror::Error;
#[derive(Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageUploadError {
    #[error("Image server error")]
    ServerError,
    #[error("Image deserialization error")]
    Deserialization,
    #[error("Image read error.")]
    Read,
    #[error("Image format error.")]
    Format,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageResult {
    format: String,
    height: u32,
    width: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageSize {
    height: u32,
    width: u32,
}