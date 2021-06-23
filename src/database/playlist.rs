use futures::StreamExt;
use mongodb::{
    bson::{self, doc, oid::ObjectId, Document},
    options::{FindOneAndUpdateOptions, ReturnDocument},
    Collection,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PlaylistDraft {
    pub(crate) tracklist: Vec<Document>,
    pub(crate) trackcount: i64,
    pub(crate) tag: Option<String>,
}

impl PlaylistDraft {
    pub fn get_doc(&self) -> Document {
        bson::to_document(self).unwrap()
    }

    pub fn get_json(&self, id: String) -> PlaylistJson {
        let tracklist = self
            .tracklist
            .iter()
            .map(|track| {
                bson::from_document::<Track>(track.clone())
                    .unwrap()
                    .get_json()
            })
            .collect::<Vec<TrackJson>>();
        PlaylistJson {
            tracklist,
            trackcount: self.trackcount,
            tag: self.tag.clone(),
            id,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Playlist {
    pub(crate) tracklist: Vec<Track>,
    pub(crate) trackcount: i64,
    pub(crate) tag: Option<String>,
    #[serde(rename = "_id")]
    pub id: ObjectId,
}

impl Playlist {
    pub fn get_json(&self) -> PlaylistJson {
        let tracklist = self
            .tracklist
            .iter()
            .map(|track| track.get_json())
            .collect::<Vec<TrackJson>>();

        PlaylistJson {
            tracklist,
            trackcount: self.trackcount,
            tag: self.tag.clone(),
            id: self.id.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PlaylistJson {
    pub tracklist: Vec<TrackJson>,
    pub trackcount: i64,
    pub tag: Option<String>,
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct Track {
    pub(crate) url: String,
    pub(crate) track_id: ObjectId,
}

impl Track {
    pub fn get_json(&self) -> TrackJson {
        TrackJson {
            url: self.url.clone(),
            track_id: self.track_id.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TrackJson {
    pub url: String,
    pub track_id: String,
}

pub struct PlaylistManager {
    collection: Collection,
}

impl PlaylistManager {
    pub fn init(collection: Collection) -> PlaylistManager {
        PlaylistManager { collection }
    }

    //need fix about cursor
    pub async fn get_all(&self) -> Vec<PlaylistJson> {
        let collection = self.collection.find(None, None).await.unwrap();

        let collection = collection
            .collect::<Vec<Result<Document, mongodb::error::Error>>>()
            .await;

        //convert vec into a different one
        collection
            .iter()
            .map(|document| {
                let document = document.clone().unwrap();

                bson::from_document::<Playlist>(document)
                    .unwrap()
                    .get_json()
            })
            .collect::<Vec<PlaylistJson>>()
    }

    pub async fn create_one(&self, playlist: PlaylistDraft) -> PlaylistJson {
        let id = self
            .collection
            .insert_one(playlist.get_doc(), None)
            .await
            .unwrap()
            .inserted_id;

        let id = id.as_object_id().unwrap().to_string();
        playlist.get_json(id)
    }

    pub async fn delete_one(&self, id: ObjectId) -> PlaylistJson {
        let deleted_playlist = self
            .collection
            .find_one_and_delete(doc! { "_id": id }, None)
            .await
            .unwrap();

        bson::from_document::<Playlist>(deleted_playlist.unwrap())
            .unwrap()
            .get_json()
    }

    pub async fn get_one(&self, id: ObjectId) -> PlaylistJson {
        let playlist = self
            .collection
            .find_one(doc! { "_id": id}, None)
            .await
            .unwrap();

        bson::from_document::<Playlist>(playlist.unwrap())
            .unwrap()
            .get_json()
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

        Ok(self.tracklist_update(increase_count).await)
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

        Ok(self.tracklist_update(decrease_count).await)
    }

    //return playlist with updated tracklist and count
    async fn tracklist_update(&self, count_update: Document) -> PlaylistJson {
        let filter = doc! { "_id": self.playlist_id.clone() };
        let option = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .build();

        let playlist = self
            .collection
            .find_one_and_update(filter, count_update, option)
            .await
            .unwrap()
            .unwrap();

        bson::from_document::<Playlist>(playlist)
            .unwrap()
            .get_json()
    }
}
