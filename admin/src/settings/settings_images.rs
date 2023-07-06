use leptos::*;
use leptos_router::ActionForm;
use serde::{Deserialize, Serialize};

use crate::{form::Input, settings::SettingsError, util::ResultAlert};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsImages {
    pub hero_width: i32,
    pub hero_height: i32,
    pub thumb_width: i32,
    pub thumb_height: i32,
}

#[component]
pub fn SettingsImagesForm(cx: Scope, settings: SettingsImages) -> impl IntoView {
    let settings_upsert = create_server_action::<SettingsImagesUpdate>(cx);
    let value = settings_upsert.value();
    let pending = settings_upsert.pending();

    view! { cx,
        <fieldset disabled=move || pending()>
            <legend>"Images"</legend>
            <ActionForm action=settings_upsert>
                <div class="Grid-fluid-2">
                    <Input
                        label="Hero width"
                        name="hero_width"
                        type_="number"
                        value=settings.hero_width.to_string()
                    />
                    <Input
                        label="Hero height"
                        name="hero_height"
                        type_="number"
                        value=settings.hero_height.to_string()
                    />
                </div>
                <div class="Grid-fluid-2">
                    <Input
                        label="Thumb width"
                        name="thumb_width"
                        type_="number"
                        value=settings.thumb_width.to_string()
                    />
                    <Input
                        label="Thumb height"
                        name="thumb_height"
                        type_="number"
                        value=settings.thumb_height.to_string()
                    />
                </div>
                <footer>
                    <input type="submit" value="SUBMIT"/>
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

#[server(SettingsImagesUpdate, "/api")]
pub async fn settings_images_update(
    cx: Scope,
    hero_width: String,
    hero_height: String,
    thumb_width: String,
    thumb_height: String,
) -> Result<Result<(), SettingsError>, ServerFnError> {
    use prisma_client::db;
    let prisma_client = crate::prisma::use_prisma(cx)?;

    let settings_saved = prisma_client
        .settings()
        .find_first(vec![])
        .select(db::settings::select!({ id }))
        .exec()
        .await
        .map_err(|e| {
            dbg!(e);
            ServerFnError::ServerError("Server error".to_string())
        })?;

    let id: String;
    if let Some(settings_saved) = settings_saved {
        id = settings_saved.id;
    } else {
        return Ok(Err(SettingsError::NotExist));
    }

    let settings_data = SettingsImages {
        hero_width: hero_width.parse::<i32>().unwrap_or(0),
        hero_height: hero_height.parse::<i32>().unwrap_or(0),
        thumb_width: thumb_width.parse::<i32>().unwrap_or(0),
        thumb_height: thumb_height.parse::<i32>().unwrap_or(0),
    };

    prisma_client
        .settings()
        .update(
            db::settings::UniqueWhereParam::IdEquals(id),
            vec![
                db::settings::hero_width::set(settings_data.hero_width),
                db::settings::hero_height::set(settings_data.hero_height),
                db::settings::thumb_width::set(settings_data.thumb_width),
                db::settings::thumb_height::set(settings_data.thumb_height),
            ],
        )
        .exec()
        .await
        .map_err(|e| {
            dbg!(e);
            ServerFnError::ServerError("Server error".to_string())
        })?;

    Ok(Ok(()))
    // if let Some(id) = id {
    // } else {
    //     let settings = prisma_client
    //         .settings()
    //         .create(
    //             settings_data.hero_width,
    //             settings_data.hero_height,
    //             settings_data.thumb_width,
    //             settings_data.thumb_height,
    //             vec![],
    //         )
    //         .exec()
    //         .await
    //         .map_err(|e| {
    //             dbg!(e);
    //             ServerFnError::ServerError("Server error".to_string())
    //         })?;

    //     dbg!(settings.clone());
    // }
}
