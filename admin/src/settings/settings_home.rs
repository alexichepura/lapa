use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{form::FormFooter, settings::SettingsError};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsHome {
    pub home_text: String,
}

#[component]
pub fn SettingsHomeForm(settings: SettingsHome) -> impl IntoView {
    let settings_site_update = ServerAction::<SettingsHomeUpdate>::new();
    let pending = settings_site_update.pending();

    view! {
        <fieldset prop:disabled=move || pending()>
            <legend>Home</legend>
            <ActionForm action=settings_site_update>
                <label>
                    <div>Text</div>
                    <textarea
                        name="home_text"
                        prop:value=settings.home_text.to_string()
                        rows="5"
                    ></textarea>
                </label>
                <FormFooter action=settings_site_update submit_text="Update home data" />
            </ActionForm>
        </fieldset>
    }
}

#[server(SettingsHomeUpdate, "/api")]
pub async fn settings_home_update(
    home_text: String,
) -> Result<Result<(), SettingsError>, ServerFnError> {
    let db = crate::server::db::use_db().await?;
    let settings = clorinde::queries::settings::settings().bind(&db).opt().await.map_err(|e| lib::emsg(e, "Settings find"))?;
    let Some(settings) = settings else {
        return Ok(Err(SettingsError::NotFound));
    };
    let id = settings.id;
    let res = clorinde::queries::settings::settings_update_home().bind(&db, &home_text, &id).await;
    tracing::debug!("Settings home updated res={res:?}");
    Ok(Ok(()))
}
