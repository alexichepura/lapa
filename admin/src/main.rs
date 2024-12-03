#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::Subscriber::builder()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    use admin::{
        app::AdminRouter,
        server::{
            auth_session_layer, file_and_error_handler, img_handler, init_prisma_client,
            leptos_routes_handler, server_fn_private, server_fn_public, session_layer, AppState,
        },
    };
    use axum::{
        routing::{get, post},
        Router,
    };
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use tracing::info;

    let leptopts = get_configuration(None).unwrap().leptos_options;
    let routes = generate_route_list(|| view! { <AdminRouter /> });
    let prisma_client = init_prisma_client().await;

    let private_app = Router::new()
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .route("/api/*fn_name", post(server_fn_private))
        .route("/auth/*fn_name", post(server_fn_public))
        .with_state(AppState {
            leptos_options: leptopts.clone(),
            prisma_client: prisma_client.clone(),
        })
        .layer(auth_session_layer(&prisma_client))
        .layer(session_layer(&prisma_client).await)
        .layer(tower_http::trace::TraceLayer::new_for_http());

    let app = Router::new()
        .merge(private_app)
        .route("/img/:img_name", get(img_handler))
        .fallback(file_and_error_handler)
        .with_state(AppState {
            leptos_options: leptopts.clone(),
            prisma_client: prisma_client.clone(),
        })
        .layer(tower_http::trace::TraceLayer::new_for_http());

    #[cfg(feature = "ratelimit")]
    let app = admin::server::ratelimit(app);
    #[cfg(feature = "compression")]
    let app = admin::server::compression(app, &leptopts.site_pkg_dir, &leptopts.site_root);

    info!("starting to listen TCP on http://{}", &leptopts.site_addr);
    let listener = tokio::net::TcpListener::bind(&leptopts.site_addr)
        .await
        .unwrap();
    info!("binded TCP listener on http://{}", &leptopts.site_addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
