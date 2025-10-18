use crate::AppState;
use crate::loaders::book::BookLoader;
use async_graphql::Request;
use async_graphql::dataloader::DataLoader;
#[cfg(debug_assertions)]
use async_graphql::http::GraphiQLSource;
use axum::Json;
use axum::extract::State;
#[cfg(debug_assertions)]
use axum::response::Html;
use axum::response::IntoResponse;
use std::sync::Arc;

#[cfg(debug_assertions)]
pub async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/gql/graphql").finish())
}

pub async fn graphql(
    State(state): State<Arc<AppState>>,
    Json(request): Json<Request>,
) -> impl IntoResponse {
    let request_with_db = request
        .data(DataLoader::new(
            BookLoader {
                db: state.db.to_owned(),
            },
            tokio::spawn,
        ))
        .data(state.db.to_owned());

    let response = state.graphql_schema.execute(request_with_db).await;
    Json(response)
}
