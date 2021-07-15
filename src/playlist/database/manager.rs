use futures::StreamExt;
use futures::TryStreamExt;
// use futures::stream::{self, StreamExt};
use mongodb::{Collection, Database, bson::{self, doc, oid::ObjectId, Document}};

use crate::{playlist::{schema::PlaylistJson, database::{Playlist, PlaylistDraft}}, shared::{ApiError}};




pub struct PlaylistManager {
    p_collection: Collection,
}

impl PlaylistManager {
    pub fn init(p_collection: Collection) -> PlaylistManager {
        PlaylistManager { p_collection }
    }
    
    //need fix about cursor
    pub async fn get_all(&self, database: &Database) -> Result<Vec<PlaylistJson>, ApiError> {
        let collection = self.p_collection.find(None, None).await?;
        let collection = collection.try_collect::<Vec<Document>>().await?;


        let mut p_list = Vec::new();
        for playlist in collection {
            
            let playlist = bson::from_document::<Playlist>(playlist)?.to_json(&self.p_collection).await?;

            p_list.push(playlist);
        }
        
        Ok(p_list)
    }
    
    pub async fn get_one(&self, id: ObjectId) -> Result<PlaylistJson, ApiError> {
        let playlist = self.p_collection.find_one(doc! { "_id": id}, None).await?;

        utils::parse_playlist(playlist)
    }

    pub async fn add_one(&self, playlist: PlaylistDraft) -> Result<PlaylistJson, ApiError> {
        let doc = bson::to_document(&playlist)?;
        let result = self
            .p_collection
            .insert_one(doc, None)
            .await?;


        let id = result.inserted_id.as_object_id().map(|id| id.to_string());
        todo!()
    }

    pub async fn remove_one(&self, id: ObjectId) -> Result<PlaylistJson, ApiError> {
        let result = self
            .p_collection
            .find_one_and_delete(doc! { "_id": id }, None)
            .await?;

        utils::parse_playlist(result)
    }

    // pub async fn add_track(&self, p_id: ObjectId, )s

}

// struct TrackList {

// }

mod utils {
    use crate::{playlist::{self, database::{Playlist}, schema::PlaylistJson}, shared::ApiError, track};
    use actix_web::Result;
    use mongodb::bson::{self, oid::ObjectId, Document};
    
    pub fn parse_playlist(playlist: Option<Document>) -> Result<PlaylistJson, ApiError> {
        match playlist {
            Some(playlist) => Ok(bson::from_document::<Playlist>(playlist)?.to_json()),
            None => Err(ApiError::ValidationError {
                info: playlist::error::Playlist::NotFound.to_string(),
            }),
        }
    }
}