use leptos::*;
use leptos_router::ActionForm;
use serde::{Deserialize, Serialize};

use crate::{
    image::ImageUploadError,
    upload::InputImage,
    util::{Pending, ResultAlert},
};

#[component]
pub fn ImageUpload(cx: Scope, post_id: String) -> impl IntoView {
    let upload_img = create_server_action::<UploadImg>(cx);
    let value = upload_img.value();
    let pending = upload_img.pending();
    let (_file_name, set_file_name) = create_signal(cx, None::<String>);
    let (save_byte_vec, set_save_byte_vec) = create_signal(cx, None::<Vec<u8>>);
    let (_save_file, set_save_file) = create_signal(cx, None::<String>);
    let (obj_url, set_obj_url) = create_signal(cx, None::<String>);
    view! { cx,
        <fieldset disabled=move || pending()>
            <legend>"Image upload"</legend>
            <div class="Grid-fluid-2">
                <div>
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
                                    serde_json::to_string(
                                            &save_byte_vec().unwrap_or_default().to_vec(),
                                        )
                                        .unwrap(),
                                )
                            }
                        />
                        <footer>
                            <input type="submit" value="Upload"/>
                            <Pending pending/>
                            <Suspense fallback=|| ()>
                                {move || match value() {
                                    None => ().into_view(cx),
                                    Some(v) => {
                                        let post_result = v
                                            .map_err(|_| ImageUploadError::ServerError)
                                            .flatten();
                                        view! { cx, <ResultAlert result=post_result/> }
                                            .into_view(cx)
                                    }
                                }}
                            </Suspense>
                        </footer>
                    </ActionForm>
                </div>
                <ImageUploadPreview obj_url/>
            </div>
        </fieldset>
    }
}

#[component]
pub fn ImageUploadPreview(cx: Scope, obj_url: ReadSignal<Option<String>>) -> impl IntoView {
    let view = move || match obj_url.get() {
        Some(url) => view! { cx, <img src=url/> }.into_view(cx),
        None => view! { cx, <p>"Upload preview"</p> }.into_view(cx),
    };
    view! { cx, <div class="ImageUploadPreview">{view}</div> }
}

#[server(UploadImg, "/api")]
pub async fn upload_img(
    cx: Scope,
    img: String,
    alt: String,
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

    let convert_settings = crate::image::ConvertSettings {
        hero_height: settings.hero_height as u32,
        hero_width: settings.hero_width as u32,
        thumb_height: settings.thumb_height as u32,
        thumb_width: settings.thumb_width as u32,
    };

    let image_upload_data = prisma_client
        .image()
        .create(alt, "".to_string(), db::post::id::equals(post_id), vec![])
        .exec()
        .await
        .map_err(|e| {
            dbg!(e);
            ServerFnError::ServerError("Server error".to_string())
        })?;

    let cursor = std::io::Cursor::new(img_bytes.clone());
    let img_reader = image::io::Reader::new(cursor.clone()).with_guessed_format();

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
    let file_path = crate::image::img_path_upload_ext(&id, &ext.to_string());
    std::fs::write(file_path.clone(), img_bytes).map_err(|e| {
        dbg!(e);
        ServerFnError::ServerError("Server error".to_string())
    })?;

    let img_decoded = img_reader.decode().unwrap();
    let height = img_decoded.height();
    let width = img_decoded.width();

    let buffered_read = std::io::BufReader::new(cursor);
    crate::image::create_image_variants_from_buf(buffered_read, img_decoded, &convert_settings, id)
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
