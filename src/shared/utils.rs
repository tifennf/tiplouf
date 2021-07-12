use crate::{playlist, shared::ApiError, track};
use actix_web::Result;
use mongodb::bson::{self, oid::ObjectId, Document};

pub fn validate_p_id(p_id: String) -> Result<ObjectId, ApiError> {
    ObjectId::with_string(&p_id).map_err(|_| ApiError::ValidationError {
        info: playlist::error::Playlist::InvalidId.to_string(),
    })
}

pub fn validate_t_id(track_id: String) -> Result<ObjectId, ApiError> {
    ObjectId::with_string(&track_id).map_err(|_| ApiError::ValidationError {
        info: playlist::error::Playlist::TrackInvalidId.to_string(),
    })
}

pub fn validate_track(track: track::TrackRequest) -> Result<Document, ApiError> {
    Ok(bson::to_document(&track.complete())?)
}

pub fn validate_id_playlist_track(
    p_id: String,
    track_id: String,
) -> Result<(ObjectId, ObjectId), ApiError> {
    let p_id = validate_p_id(p_id)?;
    let track_id = validate_t_id(track_id)?;

    Ok((p_id, track_id))
}

