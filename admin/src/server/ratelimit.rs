use axum::Router;
use tower_governor::{
    governor::GovernorConfigBuilder, key_extractor::SmartIpKeyExtractor, GovernorLayer,
};

pub fn ratelimit(app: Router) -> Router {
    let governor_conf = Box::new(
        GovernorConfigBuilder::default()
            .key_extractor(SmartIpKeyExtractor)
            .finish()
            .unwrap(),
    );
    app.layer(GovernorLayer {
        config: Box::leak(governor_conf),
    })
}
