use futures::TryStreamExt;
use mongodb::{
    bson::{self, doc, oid::ObjectId, Document},
    error,
    options::{FindOneAndUpdateOptions, ReturnDocument},
    Collection,
};

use crate::route::playlist::database::{Playlist, PlaylistDraft, PlaylistJson, TrackJson};

pub struct PlaylistManager {
    collection: Collection,
}

impl PlaylistManager {
    pub fn init(collection: Collection) -> PlaylistManager {
        PlaylistManager { collection }
    }

    //need fix about cursor
    pub async fn get_all(&self) -> Result<Vec<PlaylistJson>, error::Error> {
        let collection = self.collection.find(None, None).await.unwrap();
        let collection = collection.try_collect::<Vec<Document>>().await?;

        //convert vec into a different one
        let result = collection
            .iter()
            .map(|document| {
                let document = document.clone();

                //only way to fail is if collection is corrupt, anyway i dunno how to handle that from doc in closure
                let result = bson::from_document::<Playlist>(document)
                    .unwrap()
                    .get_json();

                result
            })
            .collect::<Vec<PlaylistJson>>();

        Ok(result)
    }

    pub async fn create_one(&self, playlist: PlaylistDraft) -> Result<PlaylistJson, error::Error> {
        let id = self
            .collection
            .insert_one(playlist.get_doc(), None)
            .await?
            .inserted_id;

        let id = id.as_object_id().map(|id| id.to_string());

        let id = match id {
            Some(id) => id,
            None => {
                let error = crate::error::no_id();

                return Err(error::Error::from(error));
            }
        };

        Ok(playlist.get_json(id))
    }

    pub async fn delete_one(&self, id: ObjectId) -> Result<PlaylistJson, error::Error> {
        let result = self
            .collection
            .find_one_and_delete(doc! { "_id": id }, None)
            .await?;

        parse_playlist(result)
    }

    pub async fn get_one(&self, id: ObjectId) -> Result<PlaylistJson, error::Error> {
        let result = self.collection.find_one(doc! { "_id": id}, None).await?;

        parse_playlist(result)
    }
}

pub struct TrackManager {
    collection: Collection,
    playlist_id: ObjectId,
}

impl TrackManager {
    pub fn init(collection: Collection, playlist_id: ObjectId) -> TrackManager {
        TrackManager {
            collection,
            playlist_id,
        }
    }

    pub async fn add_one(&self, track: Document) -> Result<PlaylistJson, mongodb::error::Error> {
        let add_track = doc! {
            "$push": {
                "tracklist": track
            }
        };

        let increase_count = doc! {
            "$inc": {
                "trackcount": 1
            }
        };

        let filter = doc! { "_id": self.playlist_id.clone()};

        self.collection
            .update_one(filter.clone(), add_track, None)
            .await?;
        let result = self.tracklist_update(increase_count).await?;

        parse_playlist(result)
    }

    pub async fn remove_one(
        &self,
        track_id: ObjectId,
    ) -> Result<PlaylistJson, mongodb::error::Error> {
        let filter = doc! { "_id": self.playlist_id.clone()};

        let remove_track = doc! {
            "$pull": {
                "tracklist": {
                    "track_id": track_id
                }
            }
        };

        let decrease_count = doc! {
            "$inc": {
                "trackcount": -1
            }
        };

        self.collection
            .update_one(filter, remove_track, None)
            .await?;
        let result = self.tracklist_update(decrease_count).await?;

        parse_playlist(result)
    }

    pub async fn get_one(&self, track_id: ObjectId) -> Result<TrackJson, mongodb::error::Error> {
        let filter = doc! { "_id": self.playlist_id.clone()};

        let result = self.collection.find_one(filter, None).await?;

        let playlist = parse_playlist(result)?;

        let track = playlist
            .tracklist
            .iter()
            .find(|track| track.track_id == track_id.to_string());

        match track.cloned() {
            Some(track) => Ok(track),
            None => {
                let error = crate::error::no_playlist();

                return Err(error::Error::from(error));
            }
        }
    }

    //return playlist with updated tracklist and count
    async fn tracklist_update(
        &self,
        count_update: Document,
    ) -> Result<Option<Document>, mongodb::error::Error> {
        let filter = doc! { "_id": self.playlist_id.clone() };
        let option = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .build();

        let result = self
            .collection
            .find_one_and_update(filter, count_update, option)
            .await?;

        Ok(result)
    }
}

fn parse_playlist(result: Option<Document>) -> Result<PlaylistJson, mongodb::error::Error> {
    let playlist = match result {
        Some(playlist) => bson::from_document::<Playlist>(playlist)?.get_json(),
        None => {
            let error = crate::error::no_playlist();

            return Err(error::Error::from(error));
        }
    };

    Ok(playlist)
}
