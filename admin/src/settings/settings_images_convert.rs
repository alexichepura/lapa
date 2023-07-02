use leptos::*;
use leptos_router::ActionForm;

use crate::{
    image::ConvertSettings,
    settings::SettingsError,
    util::{AlertDanger, AlertSuccess},
};

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
                            None => view! { cx, "" }.into_view(cx),
                            Some(v) => {
                                let post_result = v
                                    .map_err(|_| SettingsError::ServerError)
                                    .flatten();
                                match post_result {
                                    Ok(_) => view! { cx, <AlertSuccess/> }.into_view(cx),
                                    Err(e) => {
                                        view! { cx, <AlertDanger text=e.to_string()/> }
                                            .into_view(cx)
                                    }
                                }
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
    let convert_settings = ConvertSettings {
        hero_height: settings.hero_height as u32,
        hero_width: settings.hero_width as u32,
        thumb_height: settings.thumb_height as u32,
        thumb_width: settings.thumb_width as u32,
    };
    // TODO
    // crate::image::create_image_variants(&img_decoded, convert_settings, id);
    Ok(Ok(()))
}
