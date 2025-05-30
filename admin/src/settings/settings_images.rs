use leptos::prelude::*;
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
    let settings_upsert = ServerAction::<SettingsImagesUpdate>::new();
    let pending = settings_upsert.pending();

    view! {
        <fieldset prop:disabled=move || pending()>
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
                <FormFooter action=settings_upsert submit_text="Update images settings" />
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
    let db = crate::server::db::use_db().await?;
    let settings = clorinde::queries::settings::settings().bind(&db).opt().await.map_err(|e| lib::emsg(e, "Settings find"))?;
    let Some(settings) = settings else {
        return Ok(Err(SettingsError::NotFound));
    };
    let id = settings.id;
    let hero_width = hero_width.parse::<i32>().unwrap_or(0);
    let hero_height = hero_height.parse::<i32>().unwrap_or(0);
    let thumb_width = thumb_width.parse::<i32>().unwrap_or(0);
    let thumb_height = thumb_height.parse::<i32>().unwrap_or(0);
    let res = clorinde::queries::settings::settings_update_images()
        .bind(&db, &hero_height, &hero_width, &thumb_height, &thumb_width, &id).await;
    tracing::debug!("Settings update images res={res:?}");
    Ok(Ok(()))

}
