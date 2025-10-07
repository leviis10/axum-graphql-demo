use axum::Router;

mod health;

pub fn register() -> Router {
    Router::new().nest("/health", health::router())
}
