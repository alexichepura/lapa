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
    use clorinde::tokio_postgres;
    use config::ConfigError;
    use deadpool_postgres::Runtime;
    use dotenvy::dotenv;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use serde::Deserialize;
    use site::{
        routes::GenerateRouteList,
        server::{
            cdn_handler, file_and_error_handler, img_handler, leptos_routes_handler, robots_txt, server_fn_handler, AppState, MediaConfig
        },
    };
    use tracing::info;

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
    let routes = generate_route_list(|| view! { <GenerateRouteList /> });
    let pool = config
        .pg
        .create_pool(Some(Runtime::Tokio1), tokio_postgres::NoTls)
        .unwrap();

    let app = Router::new()
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .route("/api/{*fn_name}", post(server_fn_handler))
        .route("/robots.txt", get(robots_txt))
        .route("/img/{img_name}", get(img_handler))
        .route("/cdn/{img_name}", get(cdn_handler));

    // #[cfg(feature = "ratelimit")]
    // let app = site::server::ratelimit(app);
    #[cfg(feature = "compression")]
    let app = site::server::compression(app, &leptopts.site_pkg_dir, &leptopts.site_root);
    #[cfg(not(feature = "compression"))]
    let app = app.route("/pkg/{*file}", get(file_and_error_handler));

    let image_upload_path =
        std::env::var("IMAGE_UPLOAD_PATH").unwrap_or("image_upload".to_string());
    let image_convert_path =
        std::env::var("IMAGE_CONVERT_PATH").unwrap_or("image_convert".to_string());
    let media_config = MediaConfig {
        image_upload_path,
        image_convert_path,
    };

    let app = app
        // .merge(favicons)
        .fallback(file_and_error_handler)
        .with_state(AppState {
            leptos_options: leptopts.clone(),
            pool: pool.clone(),
            media_config: media_config.clone(),
        })
        .layer(tower_http::trace::TraceLayer::new_for_http());


    info!("starting to listen TCP on http://{}", &leptopts.site_addr);
    let listener = tokio::net::TcpListener::bind(&leptopts.site_addr)
        .await
        .unwrap();
    info!("binded TCP listener on http://{}", &leptopts.site_addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
