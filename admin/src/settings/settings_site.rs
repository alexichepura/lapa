use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{form::FormFooter, settings::SettingsError};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsSite {
    pub robots_txt: String,
    pub site_url: String,
}

#[component]
pub fn SettingsSiteForm(settings: SettingsSite) -> impl IntoView {
    let settings_site_update = ServerAction::<SettingsSiteUpdate>::new();
    let pending = settings_site_update.pending();

    view! {
        <fieldset prop:disabled=move || pending()>
            <legend>Site</legend>
            <ActionForm action=settings_site_update>
                <label>
                    <div>robots.txt</div>
                    <textarea
                        name="robots_txt"
                        prop:value=settings.robots_txt.to_string()
                        rows="5"
                    ></textarea>
                </label>
                <label>
                    <div>Site url</div>
                    <input
                        name="site_url"
                        prop:value=settings.site_url.to_string()
                        value=settings.site_url.to_string()
                    />
                </label>
                <FormFooter action=settings_site_update submit_text="Update site data" />
            </ActionForm>
        </fieldset>
    }
}

#[server(SettingsSiteUpdate, "/api")]
pub async fn settings_site_update(
    robots_txt: String,
    site_url: String,
) -> Result<Result<(), SettingsError>, ServerFnError> {
    let db = crate::server::db::use_db().await?;
    let settings = clorinde::queries::settings::settings().bind(&db).opt().await.map_err(|e| lib::emsg(e, "Settings find"))?;
    let Some(settings) = settings else {
        return Ok(Err(SettingsError::NotFound));
    };
    let id = settings.id;
    let res = clorinde::queries::settings::settings_update().bind(&db, &robots_txt, &site_url, &id).await;
    tracing::debug!("Settings updated res={res:?}");
    Ok(Ok(()))
}
