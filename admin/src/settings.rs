use leptos::*;
use leptos_meta::Title;
use leptos_router::ActionForm;
use serde::{Deserialize, Serialize};

use crate::{
    form::Input,
    post::PostError,
    util::{AlertDanger, AlertSuccess, Loading},
};

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsFormData {
    pub id: Option<String>,
    pub hero_width: i32,
    pub hero_height: i32,
    pub thumb_width: i32,
    pub thumb_height: i32,
}

#[component]
pub fn Settings(cx: Scope) -> impl IntoView {
    let settings = create_blocking_resource(cx, || (), move |_| get_settings(cx));

    view! { cx,
        <Title text="Settings"/>
        <h1>"Settings"</h1>
        <div class="Grid-fluid-2">
            <Suspense fallback=move || {
                view! { cx, <Loading/> }
            }>
                {move || {
                    settings
                        .read(cx)
                        .map(|settings| match settings {
                            Err(e) => view! { cx, <p>{e.to_string()}</p> }.into_view(cx),
                            Ok(settings) => {
                                view! { cx, <SettingsForm settings=settings/> }.into_view(cx)
                            }
                        })
                }}
            </Suspense>
        </div>
    }
}

#[server(GetSettings, "/api")]
pub async fn get_settings(cx: Scope) -> Result<SettingsFormData, ServerFnError> {
    let prisma_client = crate::prisma::use_prisma(cx)?;

    let settings = prisma_client
        .settings()
        .find_first(vec![])
        .exec()
        .await
        .map_err(|e| {
            dbg!(e);
            ServerFnError::ServerError("Server error".to_string())
        })?;

    Ok(match settings {
        Some(settings) => SettingsFormData {
            id: Some(settings.id),
            hero_width: settings.hero_width,
            hero_height: settings.hero_height,
            thumb_width: settings.thumb_width,
            thumb_height: settings.thumb_height,
        },
        None => SettingsFormData {
            id: None,
            hero_width: 720,
            hero_height: 1280,
            thumb_width: 360,
            thumb_height: 640,
        },
    })
}

#[component]
pub fn SettingsForm(cx: Scope, settings: SettingsFormData) -> impl IntoView {
    let settings_upsert = create_server_action::<SettingsUpsert>(cx);
    let value = settings_upsert.value();
    let pending = settings_upsert.pending();

    let id_view = if let Some(id) = &settings.id {
        view! { cx, <input type="hidden" name="id" value=id/> }.into_view(cx)
    } else {
        ().into_view(cx)
    };

    view! { cx,
        <fieldset disabled=move || pending()>
            <legend>"Images"</legend>
            <ActionForm action=settings_upsert>
                {id_view} <div class="Grid-fluid-2">
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
                </div> <div class="Grid-fluid-2">
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
                </div> <footer>
                    <input type="submit" value="SUBMIT"/>
                    <Show when=move || pending() fallback=|_| ()>
                        <progress indeterminate></progress>
                    </Show>
                    <Suspense fallback=|| ()>
                        {move || match value() {
                            None => view! { cx, "" }.into_view(cx),
                            Some(v) => {
                                let post_result = v.map_err(|_| PostError::ServerError).flatten();
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

#[server(SettingsUpsert, "/api")]
pub async fn settings_upsert(
    cx: Scope,
    id: Option<String>,
    hero_width: String,
    hero_height: String,
    thumb_width: String,
    thumb_height: String,
) -> Result<Result<(), PostError>, ServerFnError> {
    use prisma_client::db;
    let prisma_client = crate::prisma::use_prisma(cx)?;

    let settings_data = SettingsFormData {
        id: id.clone(),
        hero_width: hero_width.parse::<i32>().unwrap_or(0),
        hero_height: hero_height.parse::<i32>().unwrap_or(0),
        thumb_width: thumb_width.parse::<i32>().unwrap_or(0),
        thumb_height: thumb_height.parse::<i32>().unwrap_or(0),
    };

    if let Some(id) = id {
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
    } else {
        let settings = prisma_client
            .settings()
            .create(
                settings_data.hero_width,
                settings_data.hero_height,
                settings_data.thumb_width,
                settings_data.thumb_height,
                vec![],
            )
            .exec()
            .await
            .map_err(|e| {
                dbg!(e);
                ServerFnError::ServerError("Server error".to_string())
            })?;

        dbg!(settings.clone());
    }

    Ok(Ok(()))
}
