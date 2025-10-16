use crate::{AppState, handlers};
use axum::Router;
#[cfg(debug_assertions)]
use axum::routing::get;
use axum::routing::post;
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    let router = Router::new().route("/graphql", post(handlers::graphql::graphql));

    #[cfg(debug_assertions)]
    {
        router.route("/graphiql", get(handlers::graphql::graphiql))
    }

    #[cfg(not(debug_assertions))]
    {
        router
    }
}
