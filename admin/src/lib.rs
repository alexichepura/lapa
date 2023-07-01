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
    use wasm_bindgen::JsCast;

    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    let doc = document().unchecked_into::<web_sys::HtmlDocument>();
    let cookie = doc.cookie().unwrap_or_default();
    log!("{}", cookie.clone());
    let cookie_parsed = cookie::Cookie::split_parse(cookie);
    let mut userid: Option<String> = None;
    let mut username: Option<String> = None;
    for c in cookie_parsed {
        let c = c.unwrap();
        if c.name() == "userid" {
            userid = Some(c.value().to_string());
        }
        if c.name() == "username" {
            username = Some(c.value().to_string());
        }
    }
    let user: Option<User> = if let (Some(userid), Some(username)) = (userid, username) {
        // log!("{userid}:{username}");
        Some(User {
            id: userid,
            username,
        })
    } else {
        None
    };

    leptos::mount_to_body(move |cx| {
        view! { cx, <App user=user/> }
    });
}
