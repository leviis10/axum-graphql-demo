use crate::dtos::global::ErrorResponse;
use argon2::password_hash::Error as ArgonError;
use async_graphql::Error as GraphQlError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use jsonwebtoken::errors::Error as JwtError;
use sea_orm::DbErr;
use serde::Serialize;
use std::env::VarError;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use time::error::{ComponentRange, Format as TimeFormatError};

#[derive(Serialize)]
pub enum ErrorCode {
    DbErr,
    GraphQlError,
    TimeFormatError,
    NotFound,
    ParseIntError,
    ArgonError,
    IncorrectCredentials,
    VarError,
    JwtError,
    ComponentRange,
}

pub enum AppError {
    DbErr(DbErr),
    GraphQlError(GraphQlError),
    TimeFormatError(TimeFormatError),
    NotFound(String),
    ParseIntError(ParseIntError),
    ArgonError(ArgonError),
    IncorrectCredentials(String),
    VarError(VarError),
    JwtError(JwtError),
    ComponentRange(ComponentRange),
    LoaderError(String),
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::DbErr(err) => write!(f, "{}", err),
            AppError::GraphQlError(err) => write!(f, "{}", err.message),
            AppError::TimeFormatError(err) => write!(f, "{}", err),
            AppError::NotFound(err) => write!(f, "{}", err),
            AppError::ParseIntError(err) => write!(f, "{}", err),
            AppError::ArgonError(err) => write!(f, "{}", err),
            AppError::IncorrectCredentials(err) => write!(f, "{}", err),
            AppError::VarError(err) => write!(f, "{}", err),
            AppError::JwtError(err) => write!(f, "{}", err),
            AppError::ComponentRange(err) => write!(f, "{}", err),
            AppError::LoaderError(err) => write!(f, "{}", err),
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
            AppError::ArgonError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    error_code: ErrorCode::ArgonError,
                    message: err.to_string(),
                },
            ),
            AppError::IncorrectCredentials(err) => (
                StatusCode::UNAUTHORIZED,
                ErrorResponse {
                    error_code: ErrorCode::IncorrectCredentials,
                    message: err,
                },
            ),
            AppError::VarError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    error_code: ErrorCode::VarError,
                    message: err.to_string(),
                },
            ),
            AppError::JwtError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    error_code: ErrorCode::JwtError,
                    message: err.to_string(),
                },
            ),
            AppError::ComponentRange(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    error_code: ErrorCode::ComponentRange,
                    message: err.to_string(),
                },
            ),
            AppError::LoaderError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    error_code: ErrorCode::ComponentRange,
                    message: err,
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

impl From<ArgonError> for AppError {
    fn from(err: ArgonError) -> Self {
        AppError::ArgonError(err)
    }
}

impl From<VarError> for AppError {
    fn from(err: VarError) -> Self {
        AppError::VarError(err)
    }
}

impl From<JwtError> for AppError {
    fn from(err: JwtError) -> Self {
        AppError::JwtError(err)
    }
}

impl From<ComponentRange> for AppError {
    fn from(err: ComponentRange) -> Self {
        AppError::ComponentRange(err)
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
