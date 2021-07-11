use mongodb::bson::{self, Document};

use crate::{playlist::{self, database::{Playlist, PlaylistJson}}, shared::ApiError};


pub fn parse_playlist(playlist: Option<Document>) -> Result<PlaylistJson, ApiError> {
    match playlist {
        Some(playlist) => Ok(bson::from_document::<Playlist>(playlist)?.get_json()),
        None => Err(ApiError::ValidationError {
            info: playlist::error::Playlist::NotFound.to_string(),
        }),
    }
}
