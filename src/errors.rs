use std::fmt::Display;
use derive_more::{Display};
use actix_web::{
  http::{header::ContentType, StatusCode},
  HttpResponse,
};
use serde::Serialize;

use crate::db::{self};

#[derive(Debug)]
pub struct HTTPError {
    code: HTTPErrorCode,
    reason: HTTPReasonCode,
    source: Box<dyn std::error::Error>
}

#[derive(Debug, Display)]
pub enum HTTPReasonCode {
    #[display(fmt = "record/not-found")]
    RecordNotFound,

    #[display(fmt = "record/key-too-long")]
    KeyTooLong,
}

#[derive(Debug, Display)]
pub enum HTTPErrorCode {
    #[display(fmt = "internal-error")]
    InternalError,

    #[display(fmt = "not-found")]
    NotFound,

    #[display(fmt = "bad-request")]
    BadRequest,

    #[display(fmt = "timeout")]
    Timeout,
}

#[derive(Serialize)]
pub struct ErrorOutput {
    error: ErrorDetail
}

#[derive(Serialize)]
pub struct ErrorDetail {
    code: String,
    reason: String,
    message: String,
}

impl HTTPError {
    pub fn new(code: HTTPErrorCode, reason: HTTPReasonCode, source: Box<dyn std::error::Error>) -> HTTPError {
        HTTPError { code, reason, source }
    }
}

impl Display for HTTPError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.reason)
    }
}

impl std::error::Error for HTTPError {}

impl actix_web::error::ResponseError for HTTPError {
    fn error_response(&self) -> HttpResponse {
        let error = ErrorOutput {
            error: ErrorDetail {
                code: self.to_string(),
                reason: self.reason.to_string(),
                message: self.source.to_string(),
            }
        };
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(serde_json::to_string(&error).unwrap())
    }

    fn status_code(&self) -> StatusCode {
        match self.code {
            HTTPErrorCode::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            HTTPErrorCode::BadRequest => StatusCode::BAD_REQUEST,
            HTTPErrorCode::Timeout => StatusCode::GATEWAY_TIMEOUT,
            HTTPErrorCode::NotFound => StatusCode::NOT_FOUND,
        }
    }
}

impl From<db::DbError> for HTTPError {
    fn from(err: db::DbError) -> Self {
        match err {
            db::DbError::RecordNotFound => HTTPError::new(HTTPErrorCode::NotFound, HTTPReasonCode::RecordNotFound, Box::new(err)),
            db::DbError::KeyTooLong => HTTPError::new(HTTPErrorCode::BadRequest, HTTPReasonCode::KeyTooLong, Box::new(err)),
        }
    }
}