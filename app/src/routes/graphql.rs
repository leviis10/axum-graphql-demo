use crate::{AppState, handlers};
use axum::Router;
use axum::routing::{get, post};
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    let mut router = Router::new().route("/graphql", post(handlers::graphql::graphql));

    if cfg!(debug_assertions) {
        router = router.route("/graphiql", get(handlers::graphql::graphiql));
    }

    router
}
