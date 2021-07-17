use std::collections::HashSet;

use futures::StreamExt;
use mongodb::options::FindOneAndUpdateOptions;
use mongodb::options::ReturnDocument;
use mongodb::{Collection, Database, bson::{self, doc, oid::ObjectId}};

use crate::playlist::PlaylistRequest;
use crate::shared::utils;
use crate::track::TrackManager;
use crate::track::database::TrackDraft;
use crate::{playlist::{schema::PlaylistJson, database::{Playlist}}, shared::{ApiError}};




pub struct PlaylistManager {
    collection: Collection,
    track_manager: TrackManager,
}

impl PlaylistManager {
    pub fn init(database: &Database) -> PlaylistManager {
        let collection = database.collection("playlist");
        let track_manager = TrackManager::init(database);

        PlaylistManager {
            collection,
            track_manager,
        }
    }
    
    //need fix about cursor
    pub async fn get_all(&self) -> Result<Vec<PlaylistJson>, ApiError> {
        let mut cursor = self.collection.find(None, None).await?;

        let mut p_list = Vec::new();
        while let Some(playlist) = cursor.next().await {
            let playlist = bson::from_document::<Playlist>(playlist?)?;
            let tracklist = self.track_manager.get_tracklist(playlist.id.clone()).await?;

            let playlist = playlist.to_json(tracklist);
            
            p_list.push(playlist);
        }
        
        Ok(p_list)
    }
    
    pub async fn get_one(&self, p_id: ObjectId) -> Result<Option<PlaylistJson>, ApiError> {
        let playlist = self.collection.find_one(doc! { "_id": p_id.clone()}, None).await?;
        let tracklist = self.track_manager.get_tracklist(p_id).await?;

        let playlist = playlist.map(|playlist| {
            Ok::<PlaylistJson, ApiError>(bson::from_document::<Playlist>(playlist)?.to_json(tracklist))
        }
        ).transpose()?;

        Ok(playlist)
    }

    pub async fn add_one(&self, playlist: PlaylistRequest) -> Result<PlaylistJson, ApiError> {

        let ( tracklist, playlist_draft ) = playlist.to_draft();
        
        let playlist = bson::to_document(&playlist_draft)?;
        let result = self
            .collection
            .insert_one(playlist, None)
            .await?;

        let p_id = result.inserted_id.as_object_id().ok_or(ApiError::InternalServerError("error".to_string()))?;
        let tracklist = tracklist.into_iter().map(|url| {
            TrackDraft::new(url, p_id.clone())
        }).collect::<Vec<TrackDraft>>();

        self.track_manager.add_many_track(tracklist).await?;

        let tracklist = self.track_manager.get_tracklist(p_id.clone()).await?;
        let playlist = playlist_draft.to_json(tracklist, p_id);
       
        Ok(playlist)
    }

    pub async fn remove_one(&self, p_id: ObjectId) -> Result<PlaylistJson, ApiError> {
        let tracklist = self.track_manager.remove_tracklist(p_id.clone()).await?;

        let result = self
            .collection
            .find_one_and_delete(doc! { "_id": p_id }, None)
            .await?;
        
        match result {
            Some(playlist) => Ok(bson::from_document::<Playlist>(playlist)?.to_json(tracklist)),
            None => Err(ApiError::InternalServerError("delete error".to_string()))
        }
    }

    pub async fn add_track(&self, p_id: ObjectId, tracklist: Vec<TrackDraft>) -> Result<PlaylistJson, ApiError> {
        let count = tracklist.len() as i64;
        self.track_manager.add_many_track(tracklist).await?;

        let query = doc! {"_id": p_id.clone()};
        let update = doc! {
            "$inc": {
                "trackcount": count,
            }
        };
        let options = FindOneAndUpdateOptions::builder().return_document(ReturnDocument::After).build();

        let playlist = self.collection.find_one_and_update(query, update, options).await?.ok_or(ApiError::InternalServerError("Could not update playlist".to_string()))?;
        let playlist = bson::from_document::<Playlist>(playlist)?;
        
        let tracklist = self.track_manager.get_tracklist(p_id).await?;
        Ok(playlist.to_json(tracklist))
    }

    pub async fn remove_track(&self, p_id: ObjectId, id_list: HashSet<String>) -> Result<PlaylistJson, ApiError> {

        for id in id_list {
            let id = utils::validate_t_id(&id)?;

            self.track_manager.remove_one(id).await?;
        }

        let playlist = self.get_one(p_id).await?.ok_or(ApiError::InternalServerError("Could not update playlist".to_string()))?;


        Ok(playlist)
    }
}