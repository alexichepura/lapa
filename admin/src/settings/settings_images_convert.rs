use leptos::prelude::*;

use crate::{form::FormFooter, settings::SettingsError};

#[component]
pub fn ImagesConvertView() -> impl IntoView {
    let images_convert = ServerAction::<ImagesConvert>::new();
    let pending = images_convert.pending();

    view! {
        <fieldset prop:disabled=move || pending()>
            <legend>Images convert</legend>
            <ActionForm action=images_convert>
                <FormFooter action=images_convert submit_text="Reconvert images" />
            </ActionForm>
        </fieldset>
    }
}

#[server(ImagesConvert, "/api")]
pub async fn images_convert() -> Result<Result<(), SettingsError>, ServerFnError> {
    use prisma_client::db;
    let prisma_client = crate::server::use_prisma()?;

    let settings = prisma_client
        .settings()
        .find_first(vec![])
        .select(db::settings::select!({
            hero_height
            hero_width
            thumb_height
            thumb_width
        }))
        .exec()
        .await
        .map_err(|e| lib::emsg(e, "Settings find"))?;
    let settings = settings.unwrap();
    let convert_settings = crate::image::ConvertSettings {
        hero_height: settings.hero_height as u32,
        hero_width: settings.hero_width as u32,
        thumb_height: settings.thumb_height as u32,
        thumb_width: settings.thumb_width as u32,
    };

    let images = prisma_client
        .image()
        .find_many(vec![])
        .select(db::image::select!({ id ext }))
        .exec()
        .await
        .map_err(|e| lib::emsg(e, "Image find_many"))?;

    for image_data in images {
        let path = crate::image::img_path_upload_ext(&image_data.id, &image_data.ext);
        let file = std::fs::File::open(&path);
        match file {
            Ok(file) => {
                let dynamic_image = image::open(path.clone()).unwrap();
                let buffered_read = std::io::BufReader::new(file);
                let _convert_result = crate::image::create_image_variants_from_buf(
                    buffered_read,
                    dynamic_image,
                    &convert_settings,
                    &image_data.id,
                );
            }
            Err(err) => {
                tracing::error!("path:{path}, err={err}");
            }
        }
    }

    Ok(Ok(()))
}
