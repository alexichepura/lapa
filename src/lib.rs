#![feature(result_flattening)]

cfg_if::cfg_if! {if #[cfg(feature = "ssr")] {
    pub mod server;
}}

pub mod app;
pub mod err;
pub mod home;
pub mod img;
pub mod product_list;
pub mod product_page;
pub mod post;
pub mod routes;
pub mod settings;
pub mod util;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::App;
    use leptos::prelude::*;
    use wasm_bindgen::JsValue;

    tracing_wasm::set_as_global_default();
    console_error_panic_hook::set_once();

    let settings: JsValue =
        js_sys::Reflect::get(&web_sys::window().unwrap(), &JsValue::from_str("SETTINGS"))
            .unwrap_or(JsValue::NULL);

    let settings: settings::SettingsCx =
        serde_wasm_bindgen::from_value(settings).unwrap_or_default();

    tracing::info!("SETTINGS: {:?}", settings);

    hydrate_body(move || {
        view! { <App settings=settings.clone() /> }
    });
}
