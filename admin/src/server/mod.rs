use axum::{
    body::Body as AxumBody,
    extract::{FromRef, State},
    http::Request,
    response::{IntoResponse, Response},
};
use leptos::prelude::*;
use leptos_axum::handle_server_fns_with_context;
use leptos_meta::{HashedStylesheet, Link, MetaTags, Script};

use crate::{
    app::App,
    auth::User,
    settings::{settins_db, SettingsCx},
};

use cfg_if::cfg_if;
cfg_if! {if #[cfg(feature = "ssr")] {
    cfg_if! {if #[cfg(feature = "ratelimit")] {
        pub mod ratelimit;
        pub use ratelimit::*;
    }}
    cfg_if! {if #[cfg(feature = "compression")] {
        pub mod compression;
        pub use compression::*;
    }}
}}

pub mod auth;
pub mod err;
pub mod fileserv;
pub mod prisma;
pub mod session;
pub use auth::*;
pub use err::*;
pub use fileserv::*;
pub use prisma::*;
pub use session::*;

pub fn html_shell(
    options: LeptosOptions,
    user: Option<User>,
    settings: SettingsCx,
) -> impl IntoView {
    let user_json = serde_json::to_string(&user).unwrap();
    let user_script = format!("window.USER = {user_json};");
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
                <Script>{user_script}</Script>
                <Script>{settings_script}</Script>
                <AutoReload options=options.clone() />
                <HydrationScripts options />
            </head>
            <body>
                <App settings user />
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
    pub prisma_client: ArcPrisma,
}

pub async fn server_fn_public(
    State(app_state): State<AppState>,
    auth_session: AuthSession,
    request: Request<AxumBody>,
) -> impl IntoResponse {
    handle_server_fns_with_context(
        move || {
            provide_context(app_state.prisma_client.clone());
            provide_context(auth_session.clone());
        },
        request,
    )
    .await
}

pub async fn server_fn_private(
    State(app_state): State<AppState>,
    auth_session: AuthSession,
    request: Request<AxumBody>,
) -> Response {
    if auth_session.current_user.is_none() {
        return (http::StatusCode::NOT_FOUND, "NOT FOUND").into_response();
    }
    handle_server_fns_with_context(
        move || {
            provide_context(app_state.prisma_client.clone());
            provide_context(auth_session.clone());
        },
        request,
    )
    .await
    .into_response()
}

pub async fn leptos_routes_handler(
    State(app_state): State<AppState>,
    auth_session: AuthSession,
    req: Request<AxumBody>,
) -> Response {
    let user: Option<User> = auth_session.current_user.clone();
    let prisma_client = app_state.prisma_client.clone();
    let settings = settins_db(prisma_client.clone()).await;
    let leptos_options = app_state.leptos_options;
    let handler = leptos_axum::render_app_async_with_context(
        move || {
            provide_context(app_state.prisma_client.clone());
            provide_context(auth_session.clone());
        },
        move || html_shell(leptos_options.clone(), user.clone(), settings.clone()),
    );

    let leptos_res = handler(req).await.into_response();
    leptos_res
}
