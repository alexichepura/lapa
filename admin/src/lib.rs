#![feature(result_flattening)]

cfg_if::cfg_if! {if #[cfg(feature = "ssr")] {
    pub mod axum_session_prisma;
    pub mod fileserv;
    pub mod prisma;
}}

pub mod app;
pub mod auth;
pub mod err;
pub mod form;
pub mod home;
pub mod image;
pub mod layout;
pub mod post;
pub mod routes;
pub mod settings;
pub mod upload;
pub mod util;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    use crate::auth::User;
    use leptos::*;
    use wasm_bindgen::JsValue;

    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    let user: JsValue =
        js_sys::Reflect::get(&web_sys::window().unwrap(), &JsValue::from_str("USER"))
            .unwrap_or(JsValue::NULL);
    let user: Option<User> = serde_wasm_bindgen::from_value(user).ok();
    log!("USER: {:?}", user);

    let settings: JsValue =
        js_sys::Reflect::get(&web_sys::window().unwrap(), &JsValue::from_str("SETTINGS"))
            .unwrap_or(JsValue::NULL);
    let settings: settings::SettingsCx =
        serde_wasm_bindgen::from_value(settings).unwrap_or_default();
    log!("SETTINGS: {:?}", settings);

    leptos::mount_to_body(move |cx| {
        view! { cx, <App user settings/> }
    });
}
