#![recursion_limit = "256"]

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::Subscriber::builder()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    use admin::{
        app::AdminRouter,
        server::{
            auth_session_layer, content_image_handler, file_and_error_handler, leptos_routes_handler, product_image_handler, server_fn_private, server_fn_public, session_layer, AppState, MediaConfig
        },
    };
    use axum::{
        routing::{get, post},
        Router,
    };
    use clorinde::{deadpool_postgres, tokio_postgres};
    use deadpool_postgres::Runtime;
    use dotenvy::dotenv;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use serde::Deserialize;
    use tracing::info;
    use config::ConfigError;

    dotenv().expect(".env file not found");

    #[derive(Debug, Deserialize)]
    struct Config {
        pg: deadpool_postgres::Config,
    }
    impl Config {
        pub fn from_env() -> Result<Self, ConfigError> {
            config::Config::builder()
                .add_source(config::Environment::default().separator("__"))
                .build()
                .unwrap()
                .try_deserialize()
        }
    }
    let config = Config::from_env().unwrap();

    let leptopts = get_configuration(None).unwrap().leptos_options;
    let routes = generate_route_list(|| view! { <AdminRouter /> });
    let pool = config
        .pg
        .create_pool(Some(Runtime::Tokio1), tokio_postgres::NoTls)
        .unwrap();

    let image_upload_path =
        std::env::var("IMAGE_UPLOAD_PATH").unwrap_or("image_upload".to_string());
    let media_config = MediaConfig {
        image_upload_path,
    };

    let private_app = Router::new()
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .route("/api/{*fn_name}", post(server_fn_private))
        .route("/auth/{*fn_name}", post(server_fn_public))
        .with_state(AppState {
            leptos_options: leptopts.clone(),
            pool: pool.clone(),
            media_config: media_config.clone(),
        })
        .layer(auth_session_layer(&pool))
        .layer(session_layer(&pool).await)
        .layer(tower_http::trace::TraceLayer::new_for_http());

    let app = Router::new()
        .merge(private_app)
        .route("/content-image/{image_name}", get(content_image_handler))
        .route("/product-image/{image_name}", get(product_image_handler))
        .fallback(file_and_error_handler)
        .with_state(AppState {
            leptos_options: leptopts.clone(),
            pool: pool.clone(),
            media_config: media_config.clone(),
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
