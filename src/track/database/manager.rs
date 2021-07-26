use futures::TryStreamExt;
use mongodb::{
    bson::{self, doc, oid::ObjectId, Document},
    results::InsertManyResult,
    Collection, Database,
};

use crate::{shared::ApiError, track::schema::TrackJson};

use super::{document::TrackDraft, Track};

pub struct TrackManager {
    collection: Collection,
}

impl TrackManager {
    pub fn new(database: &Database) -> TrackManager {
        let collection = database.collection("track");

        TrackManager { collection }
    }

    pub async fn get_one(&self, id: ObjectId) -> Result<Option<TrackJson>, ApiError> {
        let track = self.collection.find_one(doc! {"_id": id}, None).await?;

        track
            .map(|track| Ok(bson::from_document::<Track>(track)?.into_json()))
            .transpose()
    }

    pub async fn add_one(&self, track: TrackDraft) -> Result<Option<TrackJson>, ApiError> {
        let doc = bson::to_document(&track)?;
        let result = self.collection.insert_one(doc, None).await?;

        let id = result.inserted_id.as_object_id().map(|id| id.to_string());
        let track = id.map(|id| track.into_json(id));

        Ok(track)
    }

    pub async fn remove_one(&self, id: ObjectId) -> Result<Option<TrackJson>, ApiError> {
        let track = self
            .collection
            .find_one_and_delete(doc! { "_id": id }, None)
            .await?;

        track
            .map(|track| Ok(bson::from_document::<Track>(track)?.into_json()))
            .transpose()
    }

    pub async fn get_tracklist(&self, p_id: ObjectId) -> Result<Vec<TrackJson>, ApiError> {
        let tracklist = self.collection.find(doc! { "p_id": p_id }, None).await?;
        let tracklist = tracklist.try_collect::<Vec<Document>>().await?;

        tracklist
            .into_iter()
            .map(|doc| Ok(bson::from_document::<Track>(doc)?.into_json()))
            .collect::<Result<Vec<TrackJson>, ApiError>>()
    }

    pub async fn add_tracklist(
        &self,
        tracklist: Vec<TrackDraft>,
    ) -> Result<Vec<TrackJson>, ApiError> {
        let tracklist_as_doc = tracklist
            .iter()
            .map(|track| Ok(bson::to_document(track)?))
            .collect::<Result<Vec<Document>, ApiError>>()?;
        let result = self.collection.insert_many(tracklist_as_doc, None).await?;

        let id_list = result.inserted_ids;

        let tracklist = tracklist
            .into_iter()
            .zip(id_list.values())
            .map(|(track, id)| {
                let id = id.as_object_id()?.to_string();

                Some(track.into_json(id))
            })
            .collect::<Option<Vec<TrackJson>>>()
            .ok_or_else(ApiError::id_not_generate)?;

        Ok(tracklist)
    }

    pub async fn remove_tracklist(&self, p_id: ObjectId) -> Result<Vec<TrackJson>, ApiError> {
        let query = doc! {"p_id": p_id.clone()};
        let tracklist = self.get_tracklist(p_id).await?;
        self.collection.delete_many(query, None).await?;

        Ok(tracklist)
    }

    pub async fn add_many_track(
        &self,
        tracklist: Vec<TrackDraft>,
    ) -> Result<InsertManyResult, ApiError> {
        let tracklist = tracklist
            .iter()
            .map(|track| Ok(bson::to_document(track)?))
            .collect::<Result<Vec<Document>, ApiError>>()?;
        let result = self.collection.insert_many(tracklist, None).await?;

        Ok(result)
    }
}
