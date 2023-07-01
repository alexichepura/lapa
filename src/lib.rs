#![feature(result_flattening)]

pub mod app;
pub mod err;
pub mod fileserv;
pub mod home;
pub mod post_list;
pub mod post_page;
pub mod prisma;
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
