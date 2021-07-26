use std::{fmt::Debug, io};

use super::ApiResponse;
use actix_web::http::StatusCode;
use derive_more::Display;
use log::error;
use serde::Serialize;

#[derive(Debug, Display, Serialize)]
pub enum Ressource {
    #[display(fmt = "Playlist")]
    Playlist,
    #[display(fmt = "Track")]
    Track,
    #[display(fmt = "User")]
    User,
}

#[derive(Debug, Display, Serialize, Clone)]
pub enum ApiError {
    #[display(fmt = "Validation error: {}", self.0)]
    ValidationError(String),
    #[display(fmt = "Query error on ressource: {}", self.0)]
    QueryError(Ressource),
    #[display(fmt = "Internal server error: {}", self.0)]
    InternalServerError(String),
    #[display(fmt = "Mongodb error: {}", self.0)]
    DatabaseError(String),
}

impl ApiError {
    pub fn id_not_generate() -> mongodb::error::Error {
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
            Ressource::User => Ressource::User,
        }
    }
}

impl actix_web::error::ResponseError for ApiError {
    fn error_response(&self) -> actix_web::HttpResponse {
        // let data = self.clone();
        match self {
            ApiError::ValidationError(_) | ApiError::QueryError(_) => {
                ApiResponse::fail(Some(self.clone()), self.status_code())
            }
            ApiError::InternalServerError(_) | ApiError::DatabaseError(_) => {
                ApiResponse::fail(None, self.status_code())
            }
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::ValidationError(_) => StatusCode::BAD_REQUEST,
            ApiError::QueryError(_) => StatusCode::NOT_FOUND,
            ApiError::InternalServerError(_) => {
                error!("{}", &self);
                StatusCode::INTERNAL_SERVER_ERROR
            }
            ApiError::DatabaseError(_) => {
                error!("{}", &self);
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

impl From<mongodb::error::Error> for ApiError {
    fn from(err: mongodb::error::Error) -> Self {
        ApiError::DatabaseError(err.to_string())
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

impl From<bcrypt::BcryptError> for ApiError {
    fn from(err: bcrypt::BcryptError) -> Self {
        ApiError::InternalServerError(err.to_string())
    }
}
