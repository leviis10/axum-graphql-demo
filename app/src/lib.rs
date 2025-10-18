use crate::graphql::{Mutation, QueryRoot};
use async_graphql::{EmptySubscription, Schema};
use axum::http::HeaderName;
use sea_orm::{Database, DatabaseConnection};
use std::env;
use std::error::Error;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::signal;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod dtos;
mod entities;
mod errors;
mod graphql;
mod handlers;
mod loaders;
mod repositories;
mod routes;
mod utils;

struct AppState {
    db: DatabaseConnection,
    graphql_schema: Schema<QueryRoot, Mutation, EmptySubscription>,
}

pub async fn start() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
    tracing::info!("Starting Axum Application");

    let db_host = env::var("DB_HOST")?;
    let db_name = env::var("DB_NAME")?;
    let db_username = env::var("DB_USERNAME")?;
    let db_password = env::var("DB_PASSWORD")?;
    let db = Database::connect(format!(
        "postgres://{db_username}:{db_password}@{db_host}/{db_name}"
    ))
    .await?;
    tracing::info!("Connected to the database");

    let graphql_schema = Schema::build(QueryRoot, Mutation, EmptySubscription).finish();

    let x_request_id = HeaderName::from_static("x-request-id");
    let timeout_duration = env::var("TIMEOUT_DURATION")
        .unwrap_or(String::from("30"))
        .parse()?;
    let state = Arc::new(AppState { db, graphql_schema });

    let app = routes::register().with_state(state).layer(
        ServiceBuilder::new()
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().include_headers(true))
                    .on_response(DefaultOnResponse::new().include_headers(true)),
            )
            .layer(CompressionLayer::new())
            .layer(SetRequestIdLayer::new(
                x_request_id.clone(),
                MakeRequestUuid,
            ))
            .layer(PropagateRequestIdLayer::new(x_request_id))
            .layer(TimeoutLayer::new(Duration::from_secs(timeout_duration))),
    );

    let port = env::var("PORT").unwrap_or(String::from("8080"));
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;

    tracing::info!("{}", format!("Server is listening on {port}"));
    tracing::info!("Serving GraphQL on /gql/graphql");
    tracing::info!(
        "Axum application started successfully in {:?}",
        start_time.elapsed()
    );
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("Shutting down gracefully...");
}
