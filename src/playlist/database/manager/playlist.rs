use futures::TryStreamExt;
use mongodb::{
    bson::{self, doc, oid::ObjectId, Document},
    Collection,
};

use crate::{playlist::{database::{Playlist, PlaylistDraft, PlaylistJson, manager::utils}}, shared::ApiError};

pub struct PlaylistManager {
    collection: Collection,
}

impl PlaylistManager {
    pub fn init(collection: Collection) -> PlaylistManager {
        PlaylistManager { collection }
    }

    //need fix about cursor
    pub async fn get_all(&self) -> Result<Vec<PlaylistJson>, ApiError> {
        let collection = self.collection.find(None, None).await?;
        let collection = collection.try_collect::<Vec<Document>>().await?;

        collection
            .iter()
            .map(|document| {
                let document = document.clone();

                Ok(bson::from_document::<Playlist>(document)?.get_json())
            })
            .collect::<Result<Vec<PlaylistJson>, ApiError>>()
    }

    pub async fn create_one(&self, playlist: PlaylistDraft) -> Result<PlaylistJson, ApiError> {
        let id = self
            .collection
            .insert_one(playlist.get_doc(), None)
            .await?
            .inserted_id;

        let id = id.as_object_id().map(|id| id.to_string());
        match id {
            Some(id) => Ok(playlist.get_json(id)),
            None => Err(ApiError::id_not_generate()),
        }
    }

    pub async fn delete_one(&self, id: ObjectId) -> Result<PlaylistJson, ApiError> {
        let result = self
            .collection
            .find_one_and_delete(doc! { "_id": id }, None)
            .await?;

        utils::parse_playlist(result)
    }

    pub async fn get_one(&self, id: ObjectId) -> Result<PlaylistJson, ApiError> {
        let result = self.collection.find_one(doc! { "_id": id}, None).await?;

        utils::parse_playlist(result)
    }
}