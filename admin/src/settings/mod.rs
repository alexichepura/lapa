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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsData {
    pub robots_txt: String,
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
        }
    }
}

#[component]
pub fn Settings(cx: Scope) -> impl IntoView {
    let settings = create_blocking_resource(
        cx,
        || (),
        move |_| async move {
            get_settings(cx)
                .await
                .map_err(|_| SettingsError::ServerError)
                .flatten()
        },
    );

    view! { cx,
        <Title text="Settings"/>
        <h1>"Settings"</h1>
        <Suspense fallback=move || {
            view! { cx, <Loading/> }
        }>
            {move || {
                settings
                    .read(cx)
                    .map(|settings| match settings {
                        Err(e) => view! { cx, <p>{e.to_string()}</p> }.into_view(cx),
                        Ok(settings) => {
                            view! { cx,
                                <div class="Grid-fluid-2">
                                    <SettingsHomeForm settings=SettingsHome::from(&settings)/>
                                    <SettingsSiteForm settings=SettingsSite::from(&settings)/>
                                </div>
                                <div class="Grid-fluid-2">
                                    <SettingsImagesForm settings=SettingsImages::from(&settings)/>
                                    <ImagesConvertView/>
                                </div>
                            }
                                .into_view(cx)
                        }
                    })
            }}
        </Suspense>
    }
}

type SettingsResult = Result<SettingsData, SettingsError>;
#[server(GetSettings, "/api")]
pub async fn get_settings(cx: Scope) -> Result<SettingsResult, ServerFnError> {
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
        Some(settings) => Ok(SettingsData {
            robots_txt: settings.robots_txt,
            hero_width: settings.hero_width,
            hero_height: settings.hero_height,
            thumb_width: settings.thumb_width,
            thumb_height: settings.thumb_height,
            home_text: settings.home_text,
        }),
        None => {
            crate::err::serverr_404(cx);
            Err(SettingsError::NotFound)
        }
    })
}
