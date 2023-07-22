pub mod auth;
pub mod err;
pub mod session;
use crate::{app::App, auth::User, settings::settins_db};
pub use auth::*;
use axum::{
    body::Body as AxumBody,
    extract::{FromRef, Path, RawQuery, State},
    http::Request,
    response::{IntoResponse, Response},
};
pub use err::*;
use http::HeaderMap;
use leptos::LeptosOptions;
use leptos::*;
use leptos_axum::handle_server_fns_with_context;
pub use session::*;

#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub prisma_client: std::sync::Arc<prisma_client::db::PrismaClient>,
}

pub async fn server_fn_public(
    State(app_state): State<AppState>,
    auth_session: AuthSession,
    path: Path<String>,
    headers: HeaderMap,
    raw_query: RawQuery,
    request: Request<AxumBody>,
) -> impl IntoResponse {
    handle_server_fns_with_context(
        path,
        headers,
        raw_query,
        move |cx| {
            provide_context(cx, app_state.prisma_client.clone());
            provide_context(cx, auth_session.clone());
        },
        request,
    )
    .await
}

pub async fn server_fn_private(
    State(app_state): State<AppState>,
    auth_session: AuthSession,
    path: Path<String>,
    headers: HeaderMap,
    raw_query: RawQuery,
    request: Request<AxumBody>,
) -> Response {
    if auth_session.current_user.is_none() {
        return (http::StatusCode::NOT_FOUND, "NOT FOUND").into_response();
    }
    handle_server_fns_with_context(
        path,
        headers,
        raw_query,
        move |cx| {
            provide_context(cx, app_state.prisma_client.clone());
            provide_context(cx, auth_session.clone());
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
        move |cx| {
            provide_context(cx, app_state.prisma_client.clone());
            provide_context(cx, auth_session.clone());
        },
        move |cx| view! { cx, <App user=user.clone() settings=settings.clone()/> },
    );

    let leptos_res = handler(req).await.into_response();
    leptos_res
}
