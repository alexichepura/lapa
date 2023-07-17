use leptos::{use_context, Scope};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SettingsCx {
    pub site_url: String,
    pub hero_width: i32,
    pub hero_height: i32,
    pub thumb_width: i32,
    pub thumb_height: i32,
}

pub fn use_settings(cx: Scope) -> SettingsCx {
    let settings = use_context::<SettingsCx>(cx).expect("to have found the settings provided");
    settings
}
pub fn use_site_url(cx: Scope) -> String {
    let settings = use_context::<SettingsCx>(cx).expect("to have found the settings provided");
    settings.site_url
}
