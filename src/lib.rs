#![feature(result_flattening)]

cfg_if::cfg_if! {if #[cfg(feature = "ssr")] {
    pub mod fileserv;
    pub mod prisma;
    pub mod server;
}}

pub mod app;
pub mod err;
pub mod home;
pub mod img;
pub mod post_list;
pub mod post_page;
pub mod routes;
pub mod util;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    use leptos::*;
    use wasm_bindgen::JsValue;

    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    let settings: JsValue =
        js_sys::Reflect::get(&web_sys::window().unwrap(), &JsValue::from_str("SETTINGS"))
            .unwrap_or(JsValue::NULL);

    let settings: SettingsCx = serde_wasm_bindgen::from_value(settings).unwrap_or_default();

    log!("SETTINGS: {:?}", settings);

    leptos::mount_to_body(move |cx| {
        view! { cx, <App settings/> }
    });
}
