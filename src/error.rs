use actix_web::{error, http::StatusCode, HttpResponse, HttpResponseBuilder};
use serde::{Deserialize, Serialize};
use crate::hcaptcha::HcaptchaError;

pub type Result<T, E = ApiError> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("invalid request")]
    InvalidRequest,
    #[error("request resource not found")]
    NotFound,
    #[error("try to create already exists resource")]
    Duplicate,
    #[error("permission is not sufficient to execute request")]
    PermissionDenied,
    #[error("captcha challenge failed, {0}")]
    ChallengeFailure(#[from] HcaptchaError),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ErrorResponse {
    err: String,
}

impl error::ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match *self {
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::Duplicate => StatusCode::CONFLICT,
            ApiError::PermissionDenied => StatusCode::FORBIDDEN,
            ApiError::ChallengeFailure(_) => StatusCode::FORBIDDEN,
            _ => StatusCode::BAD_REQUEST,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code()).json(ErrorResponse::from(self))
    }
}

impl From<&ApiError> for ErrorResponse {
    fn from(e: &ApiError) -> Self {
        Self { err: e.to_string() }
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(e: sqlx::Error) -> Self {
        use sqlx::Error::*;
        error!("{}", e);
        match e {
            RowNotFound => ApiError::NotFound,
            Database(db_error) => {
                error!("{:?} - {}", db_error.code(), db_error.message());
                if let Some(code) = db_error.code() {
                    match code.as_ref() {
                        "23505" => ApiError::Duplicate,
                        _ => ApiError::InvalidRequest,
                    }
                } else {
                    ApiError::InvalidRequest
                }
            }
            _ => ApiError::InvalidRequest,
        }
    }
}
