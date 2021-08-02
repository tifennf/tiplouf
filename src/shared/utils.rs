use crate::{
    playlist::{database::Playlist, schema::PlaylistJson},
    shared::ApiError,
    track::TrackManager,
};
use actix_web::{HttpRequest, Result};
use futures::StreamExt;
use mongodb::{
    bson::{self, oid::ObjectId},
    Cursor,
};

use super::{error::{InternalServerError, ValidationError}, middleware::SessionInfo};

pub fn validate_p_id(p_id: &str) -> Result<ObjectId, ApiError> {
    ObjectId::with_string(p_id)
        .map_err(|_| ApiError::ValidationError(ValidationError::PlaylistId))
}

pub fn validate_t_id(track_id: &str) -> Result<ObjectId, ApiError> {
    ObjectId::with_string(track_id).map_err(|_| 
        ApiError::ValidationError(ValidationError::TrackId)
    )
}

pub fn extract_user_id(req: &HttpRequest) -> Result<ObjectId, ApiError> {
    let user_id = req
        .extensions()
        .get::<SessionInfo>()
        .ok_or(ApiError::InternalServerError(InternalServerError::ExtensionMissing))?
        .user_id
        .clone();

    Ok(user_id)
}

pub async fn create_p_list(
    track_manager: &TrackManager,
    cursor: &mut Cursor,
) -> Result<Vec<PlaylistJson>, ApiError> {
    let mut p_list = Vec::new();
    while let Some(playlist) = cursor.next().await {
        let playlist = bson::from_document::<Playlist>(playlist?)?;
        let tracklist = track_manager.get_tracklist(playlist.p_id.clone()).await?;

        let playlist = playlist.into_json(tracklist);

        p_list.push(playlist);
    }

    Ok(p_list)
}
