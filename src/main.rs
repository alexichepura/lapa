#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::Subscriber::builder()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    use axum::{
        routing::{get, post},
        Router,
    };
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use site::{
        routes::GenerateRouteList,
        server::{
            dbuild, file_and_error_handler, img_handler, leptos_routes_handler, server_fn_handler,
            AppState,
        },
    };
    use tracing::info;

    let db = dbuild().await.unwrap(); // TODO ununwrap

    let leptopts = get_configuration(None).unwrap().leptos_options;
    let routes = generate_route_list(|| view! { <GenerateRouteList /> });
    // let prisma_client = init_prisma_client().await;
    let app = Router::new()
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .route("/api/{*fn_name}", post(server_fn_handler))
        // .route("/robots.txt", get(robots_txt))
        .route("/img/{img_name}", get(img_handler))
        .fallback(file_and_error_handler)
        .with_state(AppState {
            leptos_options: leptopts.clone(),
            db: db.into(),
        })
        .layer(tower_http::trace::TraceLayer::new_for_http());

    #[cfg(feature = "ratelimit")]
    let app = site::server::ratelimit(app);
    #[cfg(feature = "compression")]
    let app = site::server::compression(app, &leptopts.site_pkg_dir, &leptopts.site_root);

    info!("starting to listen TCP on http://{}", &leptopts.site_addr);
    let listener = tokio::net::TcpListener::bind(&leptopts.site_addr)
        .await
        .unwrap();
    info!("binded TCP listener on http://{}", &leptopts.site_addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
