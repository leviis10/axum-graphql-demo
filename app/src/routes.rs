use crate::AppState;
use axum::Router;
use std::sync::Arc;

mod graphql;
mod health;

pub fn register() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/health", health::router())
        .nest("/gql", graphql::router())
}
