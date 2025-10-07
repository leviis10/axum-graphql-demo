use axum::http::HeaderName;
use std::env;
use std::error::Error;
use std::time::Duration;
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
mod errors;
mod handlers;
mod routes;

pub async fn start() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let x_request_id = HeaderName::from_static("x-request-id");
    let timeout_duration = env::var("TIMEOUT_DURATION")
        .unwrap_or(String::from("30"))
        .parse()?;

    let app = routes::register().layer(
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
