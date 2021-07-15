use futures::{StreamExt, TryStreamExt};
use mongodb::{Collection, Database, bson::{self, Document, doc, oid::ObjectId}};

use crate::{shared::ApiError, track::{TrackRequest, schema::TrackJson}};

use super::{Track, document::TrackDraft};

pub struct TrackManager {
    collection: Collection,
}

impl TrackManager {
    pub async fn init(collection: Collection) -> TrackManager {
        TrackManager {
            collection,
        }
    }

    pub async fn get_one(&self, id: ObjectId) -> Result<Option<TrackJson>, ApiError> {
        let track = self.collection.find_one(doc! {"_id": id}, None).await?;
        let track = match track {
            Some(track) => Some(bson::from_document::<Track>(track)?.to_json()),
            None => None,
        };
        
        Ok(track)
    }

    pub async fn add_one(&self, track: TrackDraft) -> Result<Option<TrackJson>, ApiError> {
        let doc = bson::to_document(&track)?;
        let result = self.collection.insert_one(doc, None).await?;
        
        let id = result.inserted_id.as_object_id().map(|id| id.to_string());

        let track = match id {
            Some(id) => Some(track.to_json(id)),
            None => None,
        };

        Ok(track)
    }

    pub async fn remove_one(&self, id: ObjectId) -> Result<Option<TrackJson>, ApiError> {
        let track = self.collection.find_one_and_delete(doc!{ "_id": id }, None).await?;
        let track = match track {
            Some(track) => Some(bson::from_document::<Track>(track)?.to_json()),
            None => None,
        };

        Ok(track)
    }

    pub async fn get_tracklist(&self, p_id: ObjectId) -> Result<Vec<TrackJson>, ApiError> {
        let tracklist = self.collection.find(doc! { "p_id": p_id }, None).await?;
        let mut tracklist = tracklist.try_collect::<Vec<Document>>().await?;

        let tracklist = tracklist.drain(0..).map(|doc| Ok(bson::from_document::<Track>(doc)?.to_json())).collect::<Result<Vec<TrackJson>, ApiError>>()?;
        
        Ok(tracklist)
    }

    // pub async fn add_tracklist(&self, tracklist: Vec)
} 




// impl TrackManager {
//     pub fn init(collection: Collection, playlist_id: ObjectId) -> TrackManager {
//         TrackManager {
//             collection,
//             playlist_id,
//         }
//     }
    
//     pub async fn get_one(&self, track_id: ObjectId) -> Result<TrackJson, ApiError> {
//         let filter = doc! { "_id": self.playlist_id.clone()};
    
//         let result = self.collection.find_one(filter, None).await?;
//         let playlist = parse_playlist(result)?;
    
//         let track = playlist
//             .tracklist
//             .iter()
//             .find(|track| track.track_id == track_id.to_string());
    
//         match track.cloned() {
//             Some(track) => Ok(track),
//             None => Err(ApiError::ValidationError {
//                 info: playlist::error::Playlist::TrackNotFound.to_string(),
//             }),
//         }
//     }
    
//     pub async fn add_one(&self, track: Document) -> Result<TrackJson, ApiError> {
//         let add_track = doc! {
//             "$push": {
//                 "tracklist": track
//             }
//         };

//         let increase_count = doc! {
//             "$inc": {
//                 "trackcount": 1
//             }
//         };

//         let filter = doc! { "_id": self.playlist_id.clone()};

//         self.collection
//             .update_one(filter.clone(), add_track, None)
//             .await?;
//         let result = self.tracklist_update(increase_count).await?;

//         parse_playlist(result)
//     }

//     pub async fn remove_one(&self, track_id: ObjectId) -> Result<TrackJson, ApiError> {
//         let filter = doc! { "_id": self.playlist_id.clone()};

//         let remove_track = doc! {
//             "$pull": {
//                 "tracklist": {
//                     "track_id": track_id
//                 }
//             }
//         };

//         let decrease_count = doc! {
//             "$inc": {
//                 "trackcount": -1
//             }
//         };

//         self.collection
//             .update_one(filter, remove_track, None)
//             .await?;
//         let result = self.tracklist_update(decrease_count).await?;

//         parse_playlist(result)
//     }


//     //return playlist with updated tracklist and count
//     async fn tracklist_update(&self, count_update: Document) -> Result<Option<Document>, ApiError> {
//         let filter = doc! { "_id": self.playlist_id.clone() };
//         let option = FindOneAndUpdateOptions::builder()
//             .return_document(ReturnDocument::After)
//             .build();

//         let playlist = self
//             .collection
//             .find_one_and_update(filter, count_update, option)
//             .await?;

//         Ok(playlist)
//     }
// }

// fn parse_playlist(playlist: Option<Document>) -> Result<TrackJson, ApiError> {
//     match playlist {
//         Some(playlist) => Ok(bson::from_document::<Playlist>(playlist)?.get_json()),
//         None => Err(ApiError::ValidationError {
//             info: playlist::error::Playlist::NotFound.to_string(),
//         }),
//     }
// }