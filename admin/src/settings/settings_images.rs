use leptos::*;
use leptos_router::ActionForm;
use serde::{Deserialize, Serialize};

use crate::{
    form::{FormFooter, Input},
    settings::SettingsError,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsImages {
    pub hero_width: i32,
    pub hero_height: i32,
    pub thumb_width: i32,
    pub thumb_height: i32,
}

#[component]
pub fn SettingsImagesForm(settings: SettingsImages) -> impl IntoView {
    let settings_upsert = create_server_action::<SettingsImagesUpdate>();
    let pending = settings_upsert.pending();

    view! {
        <fieldset disabled=move || pending()>
            <legend>Images</legend>
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
                <FormFooter action=settings_upsert submit_text="Update images settings"/>
            </ActionForm>
        </fieldset>
    }
}

#[server(SettingsImagesUpdate, "/api")]
pub async fn settings_images_update(
    hero_width: String,
    hero_height: String,
    thumb_width: String,
    thumb_height: String,
) -> Result<Result<(), SettingsError>, ServerFnError> {
    use prisma_client::db;
    let prisma_client = crate::server::use_prisma()?;

    let settings_saved = prisma_client
        .settings()
        .find_first(vec![])
        .select(db::settings::select!({ id }))
        .exec()
        .await
        .map_err(|e| lib::emsg(e, "Settings find"))?;

    let id: String;
    if let Some(settings_saved) = settings_saved {
        id = settings_saved.id;
    } else {
        return Ok(Err(SettingsError::NotFound));
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
            db::settings::id::equals(id),
            vec![
                db::settings::hero_width::set(settings_data.hero_width),
                db::settings::hero_height::set(settings_data.hero_height),
                db::settings::thumb_width::set(settings_data.thumb_width),
                db::settings::thumb_height::set(settings_data.thumb_height),
            ],
        )
        .exec()
        .await
        .map_err(|e| lib::emsg(e, "Settings update"))?;

    Ok(Ok(()))
}
