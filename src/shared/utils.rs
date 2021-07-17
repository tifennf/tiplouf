
use crate::{playlist, shared::ApiError};
use actix_web::Result;
use mongodb::bson::oid::ObjectId;



pub fn validate_p_id(p_id: &str) -> Result<ObjectId, ApiError> {
    ObjectId::with_string(p_id).map_err(|_| ApiError::ValidationError {
        info: playlist::error::Playlist::InvalidId.to_string(),
    })
}

pub fn validate_t_id(track_id: &str) -> Result<ObjectId, ApiError> {
    ObjectId::with_string(track_id).map_err(|_| ApiError::ValidationError {
        info: playlist::error::Playlist::TrackInvalidId.to_string(),
    })
}

