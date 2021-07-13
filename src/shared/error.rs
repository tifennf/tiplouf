use std::{fmt::Debug, io};

use super::ApiResponse;
use actix_web::http::StatusCode;
use derive_more::Display;
use log::error;
use mongodb::bson::document::ValueAccessError;
use serde::Serialize;
// use std::convert;

#[derive(Debug, Display, Serialize)]
pub enum Ressource {
    #[display(fmt = "Playlist")]
    Playlist,
    #[display(fmt = "Track")]
    Track,
}

#[derive(Debug, Display, Serialize)]
pub enum ApiError {
    #[display(fmt = "Validation error: {}", info)]
    ValidationError { info: String },
    #[display(fmt = "Query error on ressource: {}\nInfo: {}", ressource, info)]
    QueryError { ressource: Ressource, info: String },
    #[display(fmt = "Internal server error")]
    InternalServerError(String),
}

impl ApiError {
    pub fn id_not_generate() -> ApiError {
        io::Error::new(io::ErrorKind::InvalidData, "Id not generate").into()
    }
}

impl std::error::Error for Ressource {}
impl std::error::Error for ApiError {}

impl Clone for Ressource {
    fn clone(&self) -> Self {
        match self {
            Ressource::Playlist => Ressource::Playlist,
            Ressource::Track => Ressource::Track,
        }
    }
}
impl Clone for ApiError {
    fn clone(&self) -> Self {
        match self {
            ApiError::ValidationError { info } => ApiError::ValidationError { info: info.clone() },
            ApiError::QueryError { ressource, info } => ApiError::QueryError {
                ressource: ressource.clone(),
                info: info.clone(),
            },
            ApiError::InternalServerError(err) => ApiError::InternalServerError(err.clone()),
        }
    }
}

impl actix_web::error::ResponseError for ApiError {
    fn error_response(&self) -> actix_web::HttpResponse {
        // let data = self.clone();

        ApiResponse::fail(Some(self.clone()), self.status_code())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::ValidationError { info: _ } => StatusCode::BAD_REQUEST,
            ApiError::QueryError {
                ressource: _,
                info: _,
            } => StatusCode::NOT_FOUND,
            ApiError::InternalServerError(err) => {
                error!("DatabaseError: {}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

impl From<io::Error> for ApiError {
    fn from(err: io::Error) -> Self {
        ApiError::InternalServerError(err.to_string())
    }
}
impl From<mongodb::error::Error> for ApiError {
    fn from(err: mongodb::error::Error) -> Self {
        ApiError::InternalServerError(err.to_string())
    }
}

impl From<mongodb::bson::de::Error> for ApiError {
    fn from(err: mongodb::bson::de::Error) -> Self {
        ApiError::InternalServerError(err.to_string())
    }
}
impl From<mongodb::bson::ser::Error> for ApiError {
    fn from(err: mongodb::bson::ser::Error) -> Self {
        ApiError::InternalServerError(err.to_string())
    }
}

impl From<ValueAccessError> for ApiError {
    fn from(err: ValueAccessError) -> Self {
        ApiError::InternalServerError(err.to_string())
    }
}

// impl From<mongodb::bson::oid::Error> for ApiError {
//     fn from(err: mongodb::bson::oid::Error) -> Self {
//         ApiError::ValidationError {
//             info: err.to_string(),
//         }
//     }
// }
