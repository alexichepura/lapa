use axum::{
    body::Body as AxumBody,
    extract::{FromRef, Path, RawQuery, State},
    http::Request,
    response::{IntoResponse, Response},
};
use http::HeaderMap;
use http::StatusCode;
use leptos::LeptosOptions;
use leptos::*;
use leptos_axum::handle_server_fns_with_context;
use prisma_client::db::{self, PrismaClient};
use std::sync::Arc;

cfg_if::cfg_if! {if #[cfg(feature = "ssr")] {
    cfg_if::cfg_if! {if #[cfg(feature = "ratelimit")] {
        pub mod ratelimit;
        pub use ratelimit::*;
    }}
    cfg_if::cfg_if! {if #[cfg(feature = "compression")] {
        pub mod compression;
        pub use compression::*;
    }}
}}

pub mod err;
pub mod fileserv;
pub mod prisma;
pub use err::*;
pub use fileserv::*;
pub use prisma::*;

use crate::{app::App, settings::settins_db};

#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub prisma_client: Arc<PrismaClient>,
}

pub async fn server_fn_handler(
    State(app_state): State<AppState>,
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
    let prisma_client = app_state.prisma_client.clone();
    tokio::spawn(async move {
        let result = app_state
            .prisma_client
            .clone()
            .ssr()
            .create(path, vec![db::ssr::user_agent::set(user_agent)])
            .exec()
            .await;
        if let Err(query_error) = result {
            dbg!(query_error);
        }
    });
    let settings = settins_db(prisma_client.clone()).await;

    let handler = leptos_axum::render_app_async_with_context(
        app_state.leptos_options.clone(),
        move |cx| {
            provide_context(cx, prisma_client.clone());
        },
        move |cx| view! { cx, <App settings=settings.clone()/> },
    );
    handler(req).await.into_response()
}

pub async fn robots_txt(State(app_state): State<AppState>) -> Result<String, (StatusCode, String)> {
    use prisma_client::db;
    let prisma_client = app_state.prisma_client;

    let settings = prisma_client
        .settings()
        .find_first(vec![])
        .select(db::settings::select!({ robots_txt }))
        .exec()
        .await
        .map_err(|e| {
            dbg!(e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Server error".to_string(),
            )
        })?;
    let settings = settings.unwrap();
    Ok(settings.robots_txt)
}
