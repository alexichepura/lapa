#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{
        routing::{get, post},
        Router,
    };
    use lapa_admin::{
        routes::GenerateRouteList,
        server::{
            auth_session_layer, file_and_error_handler, img_handler, init_prisma_client,
            leptos_routes_handler, server_fn_private, server_fn_public, session_layer, AppState,
        },
    };
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};

    simple_logger::init_with_env().expect("couldn't initialize logging");

    let leptopts = get_configuration(None).await.unwrap().leptos_options;
    let routes = generate_route_list(|cx| view! { cx, <GenerateRouteList/> }).await;
    let prisma_client = init_prisma_client().await;
    let app = Router::new()
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .route("/api/*fn_name", post(server_fn_private))
        .route("/auth/*fn_name", post(server_fn_public))
        .route("/img/:img_name", get(img_handler))
        .fallback(file_and_error_handler)
        .with_state(AppState {
            leptos_options: leptopts.clone(),
            prisma_client: prisma_client.clone(),
        })
        .layer(auth_session_layer(&prisma_client))
        .layer(session_layer(&prisma_client).await);

    #[cfg(feature = "ratelimit")]
    let app = lapa_admin::server::ratelimit(app);
    #[cfg(feature = "compression")]
    let app = lapa_admin::server::compression(app, &leptopts.site_pkg_dir, &leptopts.site_root);

    log!("listening on http://{}", &leptopts.site_addr);
    axum::Server::bind(&leptopts.site_addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
