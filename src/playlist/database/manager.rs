use std::collections::HashSet;

use futures::StreamExt;
use mongodb::{Collection, Database, bson::{self, doc, oid::ObjectId}, options::FindOptions};

use crate::playlist::PlaylistRequest;
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

        let options = FindOptions::builder().limit(50).build();
        let mut cursor = self.collection.find(None, options).await?;

        utils::create_p_list(&self.track_manager, &mut cursor).await
    }

    pub async fn get_one(&self, p_id: ObjectId) -> Result<PlaylistJson, ApiError> {
        let playlist = self
            .collection
            .find_one(doc! { "_id": p_id.clone()}, None)
            .await?;
        let tracklist = self.track_manager.get_tracklist(p_id).await?;

        playlist
            .map(|playlist| {
                Ok::<PlaylistJson, ApiError>(
                    bson::from_document::<Playlist>(playlist)?.into_json(tracklist),
                )
            })
            .transpose()?
            .ok_or_else(|| ApiError::DatabaseError("Could not find playlist".into()))
    }

    pub async fn add_one(&self, playlist: PlaylistRequest) -> Result<PlaylistJson, ApiError> {
        let (tracklist, playlist_draft) = playlist.into_draft();

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

    pub async fn remove_one(&self, p_id: ObjectId) -> Result<PlaylistJson, ApiError> {
        let tracklist = self.track_manager.remove_tracklist(p_id.clone()).await?;

        let playlist = self
            .collection
            .find_one_and_delete(doc! { "_id": p_id }, None)
            .await?;

        playlist
            .map(|playlist| {
                Ok::<PlaylistJson, ApiError>(
                    bson::from_document::<Playlist>(playlist)?.into_json(tracklist),
                )
            })
            .transpose()?
            .ok_or_else(|| ApiError::DatabaseError("Could not find deleted playlist".into()))
    }

    pub async fn add_track(
        &self,
        p_id: ObjectId,
        tracklist: Vec<TrackDraft>,
    ) -> Result<PlaylistJson, ApiError> {
        self.track_manager.add_many_track(tracklist).await?;

        self.get_one(p_id).await
    }

    pub async fn remove_track(
        &self,
        p_id: ObjectId,
        id_list: HashSet<String>,
    ) -> Result<PlaylistJson, ApiError> {
        for id in id_list {
            let id = utils::validate_t_id(&id)?;

            self.track_manager.remove_one(id).await?;
        }

        self.get_one(p_id).await
    }


    pub async fn get_tag(
        &self,
        tag: String,
    ) -> Result<Vec<PlaylistJson>, ApiError> {

        let filter = doc! {
            "tag": tag
        };
        let options = FindOptions::builder().limit(50).build();
        let mut cursor = self.collection.find(filter, options).await?;

        utils::create_p_list(&self.track_manager, &mut cursor).await

    }
}
