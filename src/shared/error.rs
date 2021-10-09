use std::{fmt::Debug, io};

use super::ApiResponse;
use actix_web::http::StatusCode;
use derive_more::Display;
use log::{error, info};
use serde::Serialize;

#[derive(Debug, Display, Serialize, Clone)]
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
    ValidationError(ValidationError),
    #[display(fmt = "Query error on ressource: {}", self.0)]
    QueryError(Ressource),
    #[display(fmt = "Internal server error: {}", self.0)]
    InternalServerError(InternalServerError),
    #[display(fmt = "Mongodb error: {}", self.0)]
    DatabaseError(DatabaseError),
}

impl ApiError {
    pub fn id_not_generate() -> mongodb::error::Error {
        io::Error::new(io::ErrorKind::InvalidData, "Id not generate").into()
    }
}

impl std::error::Error for Ressource {}
impl std::error::Error for ApiError {}

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
        info!("{}", err);
        ApiError::DatabaseError(DatabaseError::MongoDb(err.to_string()))
    }
}

impl From<mongodb::bson::de::Error> for ApiError {
    fn from(err: mongodb::bson::de::Error) -> Self {
        info!("{}", err);
        ApiError::InternalServerError(InternalServerError::Other(err.to_string()))
    }
}
impl From<mongodb::bson::ser::Error> for ApiError {
    fn from(err: mongodb::bson::ser::Error) -> Self {
        info!("{}", err);
        ApiError::InternalServerError(InternalServerError::Other(err.to_string()))
    }
}

impl From<bcrypt::BcryptError> for ApiError {
    fn from(err: bcrypt::BcryptError) -> Self {
        info!("{}", err);
        ApiError::InternalServerError(InternalServerError::Other(err.to_string()))
    }
}

#[derive(Debug, Display, Serialize, Clone)]
pub enum ValidationError {
    #[display(fmt = "Invalid playlist id")]
    PlaylistId,
    #[display(fmt = "Invalid track id")]
    TrackId,
    #[display(fmt = "You are not logged in")]
    NotLogged,
    #[display(fmt = "Cookie is missing")]
    CookieMissing,
    #[display(fmt = "Username or password does not match, try again")]
    UserIdentifier,
    #[display(fmt = "Username already taken")]
    UsernameTaken,
}

#[derive(Debug, Display, Serialize, Clone)]
pub enum InternalServerError {
    #[display(fmt = "Extension session is missing")]
    ExtensionMissing,
    #[display(fmt = "Extension session is missing")]
    SessionListMissing,
    #[display(fmt = "Other: {}", self.0)]
    Other(String),
}

#[derive(Debug, Display, Serialize, Clone)]
pub enum DatabaseError {
    #[display(fmt = "Mongodb: {}", self.0)]
    MongoDb(String),
    #[display(fmt = "Id not generated")]
    IdGeneration,
}
