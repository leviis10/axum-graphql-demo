use crate::handlers;
use axum::Router;
use axum::routing::get;

pub fn router() -> Router {
    Router::new()
        .route("/liveness", get(handlers::health::liveness))
        .route("/readiness", get(handlers::health::readiness))
}
