cfg_if::cfg_if! { if #[cfg(feature = "ssr")] {
    mod settings_db;
    pub use settings_db::*;
}}
mod settings_error;
mod settings_home;
mod settings_images;
mod settings_images_convert;
mod settings_site;
pub use settings_error::*;
pub use settings_home::*;
pub use settings_images::*;
pub use settings_images_convert::*;
pub use settings_site::*;

use leptos::*;
use leptos_meta::Title;
use serde::{Deserialize, Serialize};

use crate::util::Loading;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SettingsCx {
    pub site_url: String,
    pub hero_width: i32,
    pub hero_height: i32,
    pub thumb_width: i32,
    pub thumb_height: i32,
}
pub type SettingsSignal = RwSignal<SettingsCx>;
pub fn use_settings() -> SettingsSignal {
    use_context::<SettingsSignal>().expect("settings signal")
}
pub fn use_site_url() -> Signal<String> {
    let settings = use_settings();
    create_read_slice(settings, |state| state.site_url.clone())
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsData {
    pub robots_txt: String,
    pub site_url: String,
    pub home_text: String,
    pub hero_width: i32,
    pub hero_height: i32,
    pub thumb_width: i32,
    pub thumb_height: i32,
}

impl From<&SettingsData> for SettingsHome {
    fn from(data: &SettingsData) -> Self {
        SettingsHome {
            home_text: data.home_text.clone(),
        }
    }
}

impl From<&SettingsData> for SettingsImages {
    fn from(data: &SettingsData) -> Self {
        SettingsImages {
            hero_width: data.hero_width,
            hero_height: data.hero_height,
            thumb_width: data.thumb_width,
            thumb_height: data.thumb_height,
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

pub fn create_settings_resource() -> Resource<(), Result<SettingsData, SettingsError>> {
    let settings = create_blocking_resource(
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
        <Title text="Settings"/>
        <h1>Settings</h1>
        <Suspense fallback=move || {
            view! { <Loading/> }
        }>
            {move || {
                settings
                    .read()
                    .map(|settings| match settings {
                        Err(e) => view! { <p>{e.to_string()}</p> }.into_view(),
                        Ok(settings) => {
                            view! {
                                <div class="Grid-fluid-2">
                                    <SettingsHomeForm settings=SettingsHome::from(&settings)/>
                                    <SettingsSiteForm settings=SettingsSite::from(&settings)/>
                                </div>
                                <div class="Grid-fluid-2">
                                    <SettingsImagesForm settings=SettingsImages::from(&settings)/>
                                    <ImagesConvertView/>
                                </div>
                            }
                                .into_view()
                        }
                    })
            }}
        </Suspense>
    }
}

type SettingsResult = Result<SettingsData, SettingsError>;
#[server(GetSettings, "/api")]
pub async fn get_settings() -> Result<SettingsResult, ServerFnError> {
    let prisma_client = crate::server::use_prisma()?;

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
        Some(settings) => Ok(SettingsData {
            robots_txt: settings.robots_txt,
            site_url: settings.site_url,
            hero_width: settings.hero_width,
            hero_height: settings.hero_height,
            thumb_width: settings.thumb_width,
            thumb_height: settings.thumb_height,
            home_text: settings.home_text,
        }),
        None => {
            crate::server::serverr_404();
            Err(SettingsError::NotFound)
        }
    })
}
