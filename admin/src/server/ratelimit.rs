use axum::Router;
use axum::{error_handling::HandleErrorLayer, BoxError};
use tower::ServiceBuilder;
use tower_governor::{
    errors::display_error, governor::GovernorConfigBuilder, key_extractor::SmartIpKeyExtractor,
    GovernorLayer,
};

pub fn ratelimit(app: Router) -> Router {
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
}
