#![feature(result_flattening)]

cfg_if::cfg_if! {if #[cfg(feature = "ssr")] {
    pub mod fileserv;
    pub mod prisma;
    pub mod server;
}}

pub mod app;
pub mod err;
pub mod home;
pub mod post_list;
pub mod post_page;
pub mod routes;
pub mod util;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    use leptos::*;

    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    leptos::mount_to_body(move |cx| {
        view! { cx, <App/> }
    });
}
