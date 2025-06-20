cfg_if::cfg_if! { if #[cfg(feature = "ssr")] {
    mod settings_db;
    pub use settings_db::*;
}}
mod settings_error;
mod settings_home;
mod settings_site;
pub use settings_error::*;
pub use settings_home::*;
pub use settings_site::*;

use leptos::{either::Either, prelude::*};
use leptos_meta::Title;
use serde::{Deserialize, Serialize};

use crate::util::Loading;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SettingsCx {
    pub site_url: String,
}
pub type SettingsSignal = RwSignal<SettingsCx>;
pub fn use_settings() -> SettingsSignal {
    use_context::<SettingsSignal>().expect("settings signal")
}
pub fn use_site_url() -> String {
    let settings = use_settings();
    settings.get_untracked().site_url
}
pub fn use_site_url_signal() -> Signal<String> {
    let settings = use_settings();
    create_read_slice(settings, |state| state.site_url.clone())
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsData {
    pub robots_txt: String,
    pub site_url: String,
    pub home_text: String,
}

impl From<&SettingsData> for SettingsHome {
    fn from(data: &SettingsData) -> Self {
        SettingsHome {
            home_text: data.home_text.clone(),
        }
    }
}

impl From<&SettingsData> for SettingsSite {
    fn from(data: &SettingsData) -> Self {
        SettingsSite {
            robots_txt: data.robots_txt.clone(),
            site_url: data.site_url.clone(),
        }
    }
}

pub fn create_settings_resource() -> Resource<SettingsResult> {
    let settings = Resource::new_blocking(
        || (),
        move |_| async move {
            get_settings()
                .await
                .map_err(|_| SettingsError::ServerError)
                .flatten()
        },
    );
    settings
}

#[component]
pub fn Settings() -> impl IntoView {
    let settings = create_settings_resource();

    view! {
        <Title text="Settings" />
        <h1>Settings</h1>
        <Suspense fallback=move || {
            view! { <Loading /> }
        }>
            {move || {
                settings
                    .get()
                    .map(|settings| match settings {
                        Err(e) => Either::Left(view! { <p>{e.to_string()}</p> }),
                        Ok(settings) => {
                            Either::Right(
                                view! {
                                    <div class="Grid-fluid-2">
                                        <SettingsHomeForm settings=SettingsHome::from(&settings) />
                                        <SettingsSiteForm settings=SettingsSite::from(&settings) />
                                    </div>
                                },
                            )
                        }
                    })
            }}

        </Suspense>
    }
}

type SettingsResult = Result<SettingsData, SettingsError>;
#[server(GetSettings, "/api")]
pub async fn get_settings() -> Result<SettingsResult, ServerFnError> {
    let db = crate::server::db::use_db().await?;
    let settings = clorinde::queries::settings::settings_page()
        .bind(&db).opt()
        .await
        .map_err(|e| lib::emsg(e, "Settings find"))?;

    let Some(settings) = settings else {
        tracing::error!("settings record not found in database");
        crate::server::serverr_404();
        return Ok(Err(SettingsError::NotFound));
    };
    let settings = SettingsData {
        robots_txt: settings.robots_txt,
        site_url: settings.site_url,
        home_text: settings.home_text,
    };
    Ok(Ok(settings))
}
