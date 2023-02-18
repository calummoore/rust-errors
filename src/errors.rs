use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::Display;
use serde::Serialize;
use std::{error::Error, fmt::Display};

use crate::db::{self};

#[derive(Debug)]
pub struct HTTPError {
    code: HTTPErrorCode,
    reason: HTTPReasonCode,
    source: Option<Box<dyn Error>>,
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
    error: ErrorDetail,
}

#[derive(Serialize)]
pub struct ErrorDetail {
    code: String,
    reason: String,
    message: String,
}

impl HTTPError {
    pub fn new(
        code: HTTPErrorCode,
        reason: HTTPReasonCode,
        source: Option<Box<dyn std::error::Error>>,
    ) -> HTTPError {
        HTTPError {
            code,
            reason,
            source,
        }
    }

    // pub fn all_errors(&self) {
    //     let mut err = self;
    //     while err.source.is_some() {
    //         println!("Error: {}", self.source().unwrap());
    //     }
    // }
}

impl Display for HTTPError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.reason)
    }
}

impl std::error::Error for HTTPError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref())
    }
}

impl actix_web::error::ResponseError for HTTPError {
    fn error_response(&self) -> HttpResponse {
        eprintln!("Error: {}", self);

        // Log out each error
        let mut error: &dyn std::error::Error = self;
        while let Some(source) = error.source() {
            println!("  Caused by: {}", source);
            error = source;
        }

        let error = ErrorOutput {
            error: ErrorDetail {
                code: self.to_string(),
                reason: self.reason.to_string(),
                message: self.source.as_ref().unwrap().to_string(),
            },
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
            db::DbError::RecordNotFound { source: _ } => HTTPError::new(
                HTTPErrorCode::NotFound,
                HTTPReasonCode::RecordNotFound,
                Some(Box::new(err)),
            ),
            db::DbError::IndexerErr { source: _ } => HTTPError::new(
                HTTPErrorCode::BadRequest,
                HTTPReasonCode::KeyTooLong,
                Some(Box::new(err)),
            ),
        }
    }
}

// impl From<db::DbError> for HTTPError {
//     fn from(err: db::DbError) -> Self {
//         match err {
//             db::DbError::RecordNotFound => HTTPError::new(HTTPErrorCode::NotFound, HTTPReasonCode::RecordNotFound, Box::new(err)),
//             db::DbError::KeyTooLong => HTTPError::new(HTTPErrorCode::BadRequest, HTTPReasonCode::KeyTooLong, Box::new(err)),
//         }
//     }
// }
