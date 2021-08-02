use futures::TryStreamExt;
use mongodb::{
    bson::{self, doc, oid::ObjectId, Document},
    Collection, Database,
};

use crate::{shared::{ApiError, error::DatabaseError}, track::schema::TrackJson};

use super::{document::TrackDraft, Track};

pub struct TrackManager {
    collection: Collection,
}

impl TrackManager {
    pub fn new(database: &Database) -> TrackManager {
        let collection = database.collection("track");

        TrackManager { collection }
    }

    // pub async fn get_one(&self, id: ObjectId) -> Result<Option<TrackJson>, ApiError> {
    //     let track = self.collection.find_one(doc! {"_id": id}, None).await?;

    //     track
    //         .map(|track| Ok(bson::from_document::<Track>(track)?.into_json()))
    //         .transpose()
    // }

    // pub async fn add_one(&self, track: TrackDraft) -> Result<TrackJson, ApiError> {
    //     let doc = bson::to_document(&track)?;
    //     let result = self.collection.insert_one(doc, None).await?;

    //     let t_id = result
    //         .inserted_id
    //         .as_object_id()
    //         .ok_or_else(|| ApiError::DatabaseError("Id not generated".into()))?
    //         .to_string();

    //     Ok(track.into_json(t_id))
    // }

    pub async fn remove_one(&self, p_id: ObjectId, t_id: ObjectId) -> Result<Option<TrackJson>, ApiError> {

        let filter = doc! {
            "p_id": p_id,
            "_id": t_id,
        };

        let track = self
            .collection
            .find_one_and_delete(filter, None)
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
    pub async fn get_tracklist_with_id(&self, p_id: ObjectId) -> Result<Vec<Track>, ApiError> {
        let tracklist = self.collection.find(doc! { "p_id": p_id }, None).await?;
        let tracklist = tracklist.try_collect::<Vec<Document>>().await?;

        tracklist
            .into_iter()
            .map(|doc| Ok(bson::from_document::<Track>(doc)?))
            .collect::<Result<Vec<Track>, ApiError>>()
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
                let t_id = id.as_object_id()?.to_string();

                Some(track.into_json(t_id))
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
    ) -> Result<Vec<ObjectId>, ApiError> {
        let tracklist = tracklist
            .iter()
            .map(|track| Ok(bson::to_document(track)?))
            .collect::<Result<Vec<Document>, ApiError>>()?;
        let result = self
            .collection
            .insert_many(tracklist, None)
            .await?
            .inserted_ids;

        result
            .values()
            .map(|t_id| t_id.as_object_id().cloned())
            .collect::<Option<Vec<ObjectId>>>()
            .ok_or(ApiError::DatabaseError(DatabaseError::IdGeneration))
    }
}
