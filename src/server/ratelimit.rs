use axum::Router;
use std::sync::Arc;
use tower_governor::{
    GovernorLayer, governor::GovernorConfigBuilder, key_extractor::SmartIpKeyExtractor,
};

pub fn ratelimit(app: Router) -> Router {
    let config = Arc::new(
        GovernorConfigBuilder::default()
            .key_extractor(SmartIpKeyExtractor)
            .finish()
            .unwrap(),
    );
    app.layer(GovernorLayer { config })
}
