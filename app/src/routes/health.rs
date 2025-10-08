use crate::{AppState, handlers};
use axum::Router;
use axum::routing::get;
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/liveness", get(handlers::health::liveness))
        .route("/readiness", get(handlers::health::readiness))
}
