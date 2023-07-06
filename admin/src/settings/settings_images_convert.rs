use leptos::*;
use leptos_router::ActionForm;

use crate::{settings::SettingsError, util::ResultAlert};

#[component]
pub fn ImagesConvertView(cx: Scope) -> impl IntoView {
    let images_convert = create_server_action::<ImagesConvert>(cx);
    let value = images_convert.value();
    let pending = images_convert.pending();

    view! { cx,
        <fieldset disabled=move || pending()>
            <legend>"Images"</legend>
            <ActionForm action=images_convert>
                <footer>
                    <input type="submit" value="START CONVERSION"/>
                    <Show when=move || pending() fallback=|_| ()>
                        <progress indeterminate></progress>
                    </Show>
                    <Suspense fallback=|| ()>
                        {move || match value() {
                            None => ().into_view(cx),
                            Some(v) => {
                                let post_result = v
                                    .map_err(|_| SettingsError::ServerError)
                                    .flatten();
                                view! { cx, <ResultAlert result=post_result/>}.into_view(cx)
                            }
                        }}
                    </Suspense>
                </footer>
            </ActionForm>
        </fieldset>
    }
}

#[server(ImagesConvert, "/api")]
pub async fn images_convert(cx: Scope) -> Result<Result<(), SettingsError>, ServerFnError> {
    use prisma_client::db;
    let prisma_client = crate::prisma::use_prisma(cx)?;

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
        .unwrap();
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
        .select(db::image::select!({ id }))
        .exec()
        .await
        .unwrap();

    for image_data in images {
        let path = format!("upload/{}.jpg", image_data.id);
        let dynamic_image = image::open(path.clone());
        match dynamic_image {
            Ok(dynamic_image) => {
                crate::image::create_image_variants(
                    dynamic_image,
                    &convert_settings,
                    image_data.id,
                );
            }
            Err(image_err) => {
                dbg!((path.clone(), image_err));
            }
        }
    }

    Ok(Ok(()))
}
