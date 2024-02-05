use axum::{
    body::Body as AxumBody,
    extract::{FromRef, State},
    http::Request,
    response::{IntoResponse, Response},
};
use leptos::LeptosOptions;
use leptos::*;
use leptos_axum::handle_server_fns_with_context;

use crate::{app::App, auth::User, settings::settins_db};

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
    let handler = leptos_axum::render_app_async_with_context(
        app_state.leptos_options.clone(),
        move || {
            provide_context(app_state.prisma_client.clone());
            provide_context(auth_session.clone());
        },
        move || view! { <App user=user.clone() settings=settings.clone()/> },
    );

    let leptos_res = handler(req).await.into_response();
    leptos_res
}
