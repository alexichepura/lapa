#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{
        body::Body as AxumBody,
        extract::{Extension, State},
        http::Request,
        response::{IntoResponse, Response},
        routing::get,
        Router,
    };
    use lapa_site::{
        app::App,
        routes::GenerateRouteList,
        server::{file_and_error_handler, robots_txt, AppState},
        settings::settins_db,
    };
    use leptos::*;
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

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(|cx| view! { cx, <GenerateRouteList /> }).await;

    use axum::extract::{Path, RawQuery};
    use http::HeaderMap;
    use leptos_axum::handle_server_fns_with_context;
    async fn server_fn_handler(
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

    async fn leptos_routes_handler(
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

    let app_state = AppState {
        leptos_options: leptos_options.clone(),
        prisma_client: prisma_client.clone(),
    };

    let app = Router::new()
        // .route("/api/*fn_name", axum::routing::post(leptos_axum::handle_server_fns))
        .route(
            "/api/*fn_name",
            get(server_fn_handler).post(server_fn_handler),
        )
        .route("/robots.txt", get(robots_txt))
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .fallback(file_and_error_handler)
        .with_state(app_state)
        .layer(Extension(Arc::new(leptos_options.clone())));

    #[cfg(feature = "ratelimit")]
    let app = {
        use axum::{error_handling::HandleErrorLayer, BoxError};
        use tower::ServiceBuilder;
        use tower_governor::{
            errors::display_error, governor::GovernorConfigBuilder,
            key_extractor::SmartIpKeyExtractor, GovernorLayer,
        };

        let governor_conf = Box::new(
            GovernorConfigBuilder::default()
                .key_extractor(SmartIpKeyExtractor)
                .finish()
                .unwrap(),
        );
        app.layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|e: BoxError| async move {
                    display_error(e)
                }))
                .layer(GovernorLayer {
                    config: Box::leak(governor_conf),
                }),
        )
    };

    #[cfg(feature = "compression")]
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
