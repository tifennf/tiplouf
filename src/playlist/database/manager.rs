use std::collections::HashSet;

// use futures::StreamExt;
use mongodb::{
    bson::{self, doc, oid::ObjectId},
    options::FindOptions,
    Collection, Database,
};

use crate::{playlist::PlaylistRequest, track::TrackJson};
use crate::shared::utils;
use crate::track::database::TrackDraft;
use crate::track::TrackManager;
use crate::{
    playlist::{database::Playlist, schema::PlaylistJson},
    shared::ApiError,
};

pub struct PlaylistManager {
    collection: Collection,
    track_manager: TrackManager,
}

impl PlaylistManager {
    pub fn new(database: &Database) -> PlaylistManager {
        let collection = database.collection("playlist");
        let track_manager = TrackManager::new(database);

        PlaylistManager {
            collection,
            track_manager,
        }
    }

    //need fix about cursor
    pub async fn get_all(&self, user_id: ObjectId) -> Result<Vec<PlaylistJson>, ApiError> {
        let filter = doc! {
            "user_id": user_id,
        };
        // let options = FindOptions::builder().limit(50).build();
        let mut cursor = self.collection.find(filter, None).await?;

        utils::create_p_list(&self.track_manager, &mut cursor).await
    }

    pub async fn get_one(&self, user_id: ObjectId, p_id: ObjectId) -> Result<PlaylistJson, ApiError> {
        let filter = doc! {
            "user_id": user_id,
            "_id": p_id.clone(),
        };

        let playlist = self
            .collection
            .find_one(filter , None)
            .await?;
        let tracklist = self.track_manager.get_tracklist(p_id).await?;

        playlist.ok_or_else(|| ApiError::ValidationError("Could not find playlist, invalid ID".into()))
            .and_then(|playlist| {
                Ok::<PlaylistJson, ApiError>(
                    bson::from_document::<Playlist>(playlist)?.into_json(tracklist),
                )
            })   
    }

    pub async fn add_one(&self, user_id: ObjectId, playlist: PlaylistRequest) -> Result<PlaylistJson, ApiError> {
        let (tracklist, playlist_draft) = playlist.into_draft(user_id);

        let playlist = bson::to_document(&playlist_draft)?;
        let result = self.collection.insert_one(playlist, None).await?;

        let p_id = result
            .inserted_id
            .as_object_id()
            .ok_or_else(ApiError::id_not_generate)?;
        let tracklist = tracklist
            .into_iter()
            .map(|url| TrackDraft::new(url, p_id.clone()))
            .collect::<Vec<TrackDraft>>();

        self.track_manager.add_many_track(tracklist).await?;

        let tracklist = self.track_manager.get_tracklist(p_id.clone()).await?;
        let playlist = playlist_draft.into_json(tracklist, p_id);

        Ok(playlist)
    }

    pub async fn remove_one(&self, user_id: ObjectId, p_id: ObjectId) -> Result<PlaylistJson, ApiError> {
        let filter = doc! {
            "user_id": user_id,
            "_id": p_id.clone(),
        };

        let playlist = self
            .collection
            .find_one_and_delete(filter, None)
            .await?.ok_or_else(|| ApiError::DatabaseError("Could not find deleted playlist".into()))?;
        let tracklist = self.track_manager.remove_tracklist(p_id.clone()).await?;

        let playlist = bson::from_document::<Playlist>(playlist)?.into_json(tracklist);

        Ok(playlist)
    }

    pub async fn add_track(
        &self,
        user_id: ObjectId,
        p_id: ObjectId,
        tracklist: Vec<TrackDraft>,
    ) -> Result<PlaylistJson, ApiError> {

        let mut playlist = self.get_one(user_id, p_id).await?;
        self.track_manager.add_many_track(tracklist.clone()).await?;

        playlist.tracklist = tracklist.iter().map(|track| track.clone().into_json()).chain(playlist.tracklist.into_iter()).collect::<Vec<TrackJson>>();

        Ok(playlist)
    }

    pub async fn remove_track(
        &self,
        user_id: ObjectId,
        p_id: ObjectId,
        id_list: HashSet<String>,
    ) -> Result<PlaylistJson, ApiError> {
        self.is_owner(user_id.clone(), p_id.clone()).await?;

        for id  in id_list {
            let id = utils::validate_t_id(&id)?;
            
            self.track_manager.remove_one(id.clone()).await?;

        }

        self.get_one(user_id, p_id).await
    }

    pub async fn get_tag(&self, tag: String) -> Result<Vec<PlaylistJson>, ApiError> {
        let filter = doc! {
            "tag": tag
        };
        let options = FindOptions::builder().limit(50).build();
        let mut cursor = self.collection.find(filter, options).await?;

        utils::create_p_list(&self.track_manager, &mut cursor).await
    }

    pub async fn is_owner(&self, user_id: ObjectId, p_id: ObjectId) -> Result<(), ApiError> {
        let filter = doc! {
            "user_id": user_id,
            "_id": p_id,
        };

        self
            .collection
            .find_one(filter , None)
            .await?;

        Ok(())
    }
}
