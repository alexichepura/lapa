use axum::Router;
use tower_http::{compression::CompressionLayer, services::ServeDir};

pub fn compression(app: Router, site_pkg_dir: &str, site_root: &str) -> Router {
    // let pkg_path = "/".to_owned() + &leptos_options.site_pkg_dir;
    // let pkg_dir = leptos_options.site_root.clone() + &pkg_path;
    let pkg_path = "/".to_owned() + &site_pkg_dir;
    let pkg_dir = site_root.to_owned() + &pkg_path;
    let pkg_router = Router::new()
        .nest_service(
            &pkg_path,
            ServeDir::new(pkg_dir)
                .precompressed_br()
                .precompressed_deflate()
                .precompressed_gzip(),
            // .precompressed_zstd(), // zstd brings errors (in ff is not loading, in chrome net::ERR_ZSTD_WINDOW_SIZE_TOO_BIG)
        )
        .route_layer(CompressionLayer::new());
    app.merge(pkg_router).layer(CompressionLayer::new())
}
