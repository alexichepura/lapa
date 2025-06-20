use axum::{
    body::Body as AxumBody,
    extract::{FromRef, State},
    http::Request,
    response::{IntoResponse, Response},
};
use clorinde::queries;
use http::StatusCode;
use image_config::ImageConfig;
use leptos::prelude::*;
use leptos_axum::handle_server_fns_with_context;
use leptos_meta::{HashedStylesheet, Link, MetaTags, Script};

use cfg_if::cfg_if;
cfg_if! {if #[cfg(feature = "ratelimit")] {
    pub mod ratelimit;
    pub use ratelimit::*;
}}
cfg_if! {if #[cfg(feature = "compression")] {
    pub mod compression;
    pub use compression::*;
}}

pub mod db;
pub mod cdn;
pub mod err;
pub mod fileserv;
pub use cdn::*;
pub use err::*;
pub use fileserv::*;

use crate::{
    app::App,
    settings::{settings_db, SettingsCx},
};

pub fn html_shell(options: LeptosOptions, settings: SettingsCx) -> impl IntoView {
    let settings_json = serde_json::to_string(&settings).unwrap();
    let settings_script = format!("window.SETTINGS = {settings_json};");
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <HashedStylesheet id="leptos" options=options.clone() />
                <MetaTags />
                <Favicons />
                <Script>{settings_script}</Script>
                <AutoReload options=options.clone() />
                <HydrationScripts options />
            </head>
            <body>
                <App settings />
            </body>
        </html>
    }
}

#[component]
pub fn Favicons() -> impl IntoView {
    view! {
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico" />
        <Link rel="icon" type_="image/png" sizes="32x32" href="/favicon-32x32.png" />
        <Link rel="icon" type_="image/png" sizes="16x16" href="/favicon-16x16.png" />
        <Link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png" />
        <Link rel="manifest" href="/site.webmanifest" />
    }
}

#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub pool: clorinde::deadpool_postgres::Pool,
    pub image_config: ImageConfig,
}
pub fn use_image_config() -> Result<ImageConfig, ServerFnError> {
    use_context::<ImageConfig>()
        .ok_or("ImageConfig missing.")
        .map_err(|e| ServerFnError::new(e.to_string()))
}

pub async fn server_fn_handler(
    State(app_state): State<AppState>,
    request: Request<AxumBody>,
) -> impl IntoResponse {
    handle_server_fns_with_context(
        move || {
            provide_context(app_state.pool.clone());
        },
        request,
    )
    .await
}

pub async fn leptos_routes_handler(
    State(app_state): State<AppState>,
    req: Request<AxumBody>,
) -> Response {
    let path = req.uri().path().to_string();
    let headers = req.headers();
    let user_agent: Option<String> = match headers.get("user-agent") {
        Some(ua_header) => Some(ua_header.to_str().unwrap().to_string()),
        _ => None,
    };
    let pool = app_state.pool.clone();
    let settings = settings_db(pool.clone()).await;
    let leptos_options = app_state.leptos_options;
    let handler = leptos_axum::render_app_async_with_context(
        move || {
            provide_context(pool.clone());
        },
        move || html_shell(leptos_options.clone(), settings.clone()),
    );
    handler(req).await.into_response()
}

pub async fn robots_txt(State(app_state): State<AppState>) -> Result<String, (StatusCode, String)> {
    let client = app_state.pool.clone().get().await.unwrap();
    let robots = queries::settings::settings_robots().bind(&client).opt().await.unwrap();
    dbg!(&robots);
    let robots = robots.unwrap();
    Ok(robots)
}
