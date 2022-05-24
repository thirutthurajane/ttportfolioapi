use std::fmt::{Debug, Display, Formatter};
use serde::{Serialize, Deserialize};
use mongodb::error::{ErrorKind,Error as MongoError};
use mongodb::bson::{oid::Error as OidError, de::Error as DeError};
use actix_web::{HttpResponse, error::ResponseError};
use actix_web::body::{BoxBody};
use actix_web::http::StatusCode;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiError {
    pub err_type: ErrorType,
    pub message: Option<String>
}

#[derive(Serialize, Deserialize,Debug,Clone)]
pub enum ErrorType {
    MongoError,
    MongoOidError,
    MongoDeError
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrResonse {
    pub status_code: String,
    pub message: Option<String>
}

impl From<MongoError> for ApiError {
    fn from(err: MongoError) -> Self {
        let msg = match *err.kind {
            ErrorKind::Authentication{ message ,..} => {
                message
            },
            ErrorKind::Io(err) => {
                err.to_string()
            },
            ErrorKind::Internal { message,.. } => {
                message
            },
            ErrorKind::InvalidResponse {message, ..} => {
                message
            }
            _ => {
                "Pop is lazy to implement this error message".to_string()
            }
        };
        ApiError {
            message: Some(msg),
            err_type: ErrorType::MongoError
        }
    }
}

impl From<OidError> for ApiError {
    fn from(err: OidError) -> Self {
        ApiError {
            message: Some(err.to_string()),
            err_type: ErrorType::MongoOidError
        }
    }
}

impl From<DeError> for ApiError {
    fn from(err: DeError) -> Self {
        ApiError {
            message: Some(err.to_string()),
            err_type: ErrorType::MongoOidError
        }
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self.err_type {
            ErrorType::MongoError => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).json(ErrResonse { status_code: self.status_code().to_string(), message: self.message.clone() })
    }
}