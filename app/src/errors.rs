use crate::dtos::global::ErrorResponse;
use async_graphql::Error as GraphQlError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use sea_orm::DbErr;
use serde::Serialize;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use time::error::Format as TimeFormatError;

#[derive(Serialize)]
pub enum ErrorCode {
    DbErr,
    GraphQlError,
    TimeFormatError,
    NotFound,
    ParseIntError,
}

pub enum AppError {
    DbErr(DbErr),
    GraphQlError(GraphQlError),
    TimeFormatError(TimeFormatError),
    NotFound(String),
    ParseIntError(ParseIntError),
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::DbErr(err) => write!(f, "{}", err),
            AppError::GraphQlError(err) => write!(f, "{}", err.message),
            AppError::TimeFormatError(err) => write!(f, "{}", err),
            AppError::NotFound(err) => write!(f, "{}", err),
            AppError::ParseIntError(err) => write!(f, "{}", err),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, response) = match self {
            AppError::DbErr(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    error_code: ErrorCode::DbErr,
                    message: err.to_string(),
                },
            ),
            AppError::GraphQlError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    error_code: ErrorCode::GraphQlError,
                    message: err.message,
                },
            ),
            AppError::TimeFormatError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    error_code: ErrorCode::TimeFormatError,
                    message: err.to_string(),
                },
            ),
            AppError::NotFound(err) => (
                StatusCode::NOT_FOUND,
                ErrorResponse {
                    error_code: ErrorCode::NotFound,
                    message: err,
                },
            ),
            AppError::ParseIntError(err) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    error_code: ErrorCode::ParseIntError,
                    message: err.to_string(),
                },
            ),
        };

        (status, response).into_response()
    }
}

impl From<DbErr> for AppError {
    fn from(err: DbErr) -> Self {
        AppError::DbErr(err)
    }
}

impl From<GraphQlError> for AppError {
    fn from(err: GraphQlError) -> Self {
        AppError::GraphQlError(err)
    }
}

impl From<TimeFormatError> for AppError {
    fn from(err: TimeFormatError) -> Self {
        AppError::TimeFormatError(err)
    }
}

impl From<ParseIntError> for AppError {
    fn from(err: ParseIntError) -> Self {
        AppError::ParseIntError(err)
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
