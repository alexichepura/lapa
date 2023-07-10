use leptos::*;
use leptos_router::ActionForm;
use serde::{Deserialize, Serialize};

use crate::{form::FormFooter, settings::SettingsError};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsSite {
    pub robots_txt: String,
}

#[component]
pub fn SettingsSiteForm(cx: Scope, settings: SettingsSite) -> impl IntoView {
    let settings_site_update = create_server_action::<SettingsSiteUpdate>(cx);
    let pending = settings_site_update.pending();

    view! { cx,
        <fieldset disabled=move || pending()>
            <legend>"Site"</legend>
            <ActionForm action=settings_site_update>
                <label>
                    <div>"robots.txt"</div>
                    <textarea name="robots_txt" prop:value=settings.robots_txt.to_string() />
                </label>
                <FormFooter action=settings_site_update/>
            </ActionForm>
        </fieldset>
    }
}

#[server(SettingsSiteUpdate, "/api")]
pub async fn settings_site_update(
    cx: Scope,
    robots_txt: String,
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

    let settings_data = SettingsSite { robots_txt };

    prisma_client
        .settings()
        .update(
            db::settings::id::equals(id),
            vec![db::settings::robots_txt::set(settings_data.robots_txt)],
        )
        .exec()
        .await
        .map_err(|e| {
            dbg!(e);
            ServerFnError::ServerError("Server error".to_string())
        })?;

    Ok(Ok(()))
}
