#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{
        body::Body as AxumBody,
        extract::{Extension, FromRef, Path, RawQuery, State},
        http::Request,
        response::{IntoResponse, Response},
        routing::get,
        Router,
    };
    use axum_session::{SessionConfig, SessionLayer, SessionStore};
    use axum_session_auth::{AuthConfig, AuthSessionLayer};
    use http::HeaderMap;
    use lapa_admin::{
        app::App,
        auth::{AuthSession, User},
        axum_session_prisma::SessionPrismaPool,
        fileserv::file_and_error_handler,
        prisma::ArcPrisma,
        routes::AdminRoutesList,
    };
    use leptos::*;
    use leptos_axum::handle_server_fns_with_context;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use prisma_client::db;
    use std::sync::Arc;

    simple_logger::init_with_env().expect("couldn't initialize logging");

    let client = if let Ok(db_url) = std::env::var("DATABASE_URL") {
        db::new_client_with_url(db_url.as_str()).await
    } else {
        db::new_client().await
    };
    let prisma_client = Arc::new(client.unwrap());
    #[cfg(debug)]
    prisma_client._db_push(false).await.unwrap();

    let session_config = SessionConfig::default()
        .with_table_name("Session")
        .with_cookie_name("session");
    let auth_config = AuthConfig::<String>::default();
    let session_store =
        SessionStore::<SessionPrismaPool>::new(Some(prisma_client.clone().into()), session_config);
    session_store.initiate().await.unwrap();

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(|cx| view! { cx, <AdminRoutesList/> }).await;

    #[derive(FromRef, Debug, Clone)]
    pub struct AppState {
        pub leptos_options: LeptosOptions,
        pub prisma_client: std::sync::Arc<prisma_client::db::PrismaClient>,
    }
    async fn server_fn_public(
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

    async fn server_fn_private(
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

    async fn leptos_routes_handler(
        State(app_state): State<AppState>,
        auth_session: AuthSession,
        req: Request<AxumBody>,
    ) -> Response {
        let user: Option<User> = auth_session.current_user.clone();
        let (userid_cookie, username_cookie) = if let Some(user) = user.clone() {
            (
                format!("userid={};Path=/;", user.id),
                format!("username={};Path=/;", user.username),
            )
        } else {
            (
                format!("userid=;Max-Age=-1;Path=/;"),
                format!("username=;Max-Age=-1;Path=/;"),
            )
        };

        let handler = leptos_axum::render_app_async_with_context(
            app_state.leptos_options.clone(),
            move |cx| {
                provide_context(cx, app_state.prisma_client.clone());
                provide_context(cx, auth_session.clone());
            },
            move |cx| view! { cx, <App user=user.clone()/> },
        );

        let mut leptos_res = handler(req).await.into_response();
        leptos_res.headers_mut().append(
            http::header::SET_COOKIE,
            http::HeaderValue::from_str(&userid_cookie).unwrap(),
        );
        leptos_res.headers_mut().append(
            http::header::SET_COOKIE,
            http::HeaderValue::from_str(&username_cookie).unwrap(),
        );
        leptos_res
    }

    let app_state = AppState {
        leptos_options: leptos_options.clone(),
        prisma_client: prisma_client.clone(),
    };

    let app = Router::new()
        .route(
            "/api/*fn_name",
            get(server_fn_private).post(server_fn_private),
        )
        .route(
            "/auth/*fn_name",
            get(server_fn_public).post(server_fn_public),
        )
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .fallback(file_and_error_handler)
        .with_state(app_state)
        .layer(
            AuthSessionLayer::<User, String, SessionPrismaPool, ArcPrisma>::new(Some(
                prisma_client.clone(),
            ))
            .with_config(auth_config),
        )
        .layer(SessionLayer::new(session_store))
        .layer(Extension(Arc::new(leptos_options.clone())));

    #[cfg(not(debug_assertions))]
    let app = {
        use tower_http::{compression::CompressionLayer, services::ServeDir};
        let pkg_path = "/".to_owned() + &leptos_options.site_pkg_dir;
        let pkg_dir = leptos_options.site_root.clone() + &pkg_path;
        let pkg_router = Router::new()
            .nest_service(
                &pkg_path,
                ServeDir::new(pkg_dir)
                    .precompressed_br()
                    .precompressed_deflate()
                    .precompressed_gzip()
                    .precompressed_zstd(),
            )
            .route_layer(CompressionLayer::new());
        app.merge(pkg_router).layer(CompressionLayer::new())
    };

    log!("listening on http://{}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// #[cfg(not(feature = "ssr"))]
// pub fn main() {}
