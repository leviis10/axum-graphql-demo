use crate::dtos::health::HealthResponse;
use crate::errors::Result;
use axum::http::StatusCode;
use axum::{Json, debug_handler};

#[debug_handler]
pub async fn liveness() -> Result<(StatusCode, Json<HealthResponse>)> {
    Ok((
        StatusCode::OK,
        Json(HealthResponse {
            status: String::from("UP"),
        }),
    ))
}

pub async fn readiness() -> Result<(StatusCode, Json<HealthResponse>)> {
    Ok((
        StatusCode::OK,
        Json(HealthResponse {
            status: String::from("UP"),
        }),
    ))
}
