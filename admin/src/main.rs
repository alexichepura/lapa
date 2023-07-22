#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{extract::Extension, routing::get, Router};
    use lapa_admin::{
        routes::GenerateRouteList,
        server::{
            auth_session_layer, file_and_error_handler, leptos_routes_handler, server_fn_private,
            server_fn_public, session_layer, AppState,
        },
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
    let routes = generate_route_list(|cx| view! { cx, <GenerateRouteList/> }).await;

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
        .layer(auth_session_layer(prisma_client.clone()))
        .layer(session_layer(prisma_client.clone()).await)
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
