use crate::{playlist::{self, database::Playlist, schema::PlaylistJson}, shared::ApiError, track::TrackManager};
use actix_web::Result;
use futures::StreamExt;
use mongodb::{Cursor, bson::{self, oid::ObjectId}};

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

pub async fn create_p_list(track_manager: &TrackManager, cursor: &mut Cursor) -> Result<Vec<PlaylistJson>, ApiError> {

    let mut p_list = Vec::new();
    while let Some(playlist) = cursor.next().await {
        let playlist = bson::from_document::<Playlist>(playlist?)?;
        let tracklist = track_manager
            .get_tracklist(playlist.id.clone())
            .await?;

        let playlist = playlist.into_json(tracklist);

        p_list.push(playlist);
    }

    Ok(p_list)
}