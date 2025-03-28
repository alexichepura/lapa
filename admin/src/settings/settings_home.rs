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

    prisma_client
        .settings()
        .update(
            db::settings::id::equals(id),
            vec![db::settings::home_text::set(home_text)],
        )
        .exec()
        .await
        .map_err(|e| lib::emsg(e, "Settings update"))?;

    Ok(Ok(()))
}
