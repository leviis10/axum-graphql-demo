use crate::dtos::global::ErrorResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Serialize)]
pub enum ErrorCode {
    Unidentified,
}

pub enum AppError {}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            ErrorResponse {
                error_code: ErrorCode::Unidentified,
                message: String::from("Something Went Wrong"),
            },
        )
            .into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
