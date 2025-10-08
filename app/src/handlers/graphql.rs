use crate::AppState;
use async_graphql::Request;
use async_graphql::http::GraphiQLSource;
use axum::Json;
use axum::extract::State;
use axum::response::{Html, IntoResponse};
use std::sync::Arc;

#[cfg(debug_assertions)]
pub async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/gql/graphql").finish())
}

pub async fn graphql(
    State(state): State<Arc<AppState>>,
    Json(request): Json<Request>,
) -> impl IntoResponse {
    let request_with_db = request.data(state.db.clone());

    let response = state.graphql_schema.execute(request_with_db).await;
    Json(response)
}
