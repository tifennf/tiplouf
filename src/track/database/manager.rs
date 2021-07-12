use mongodb::{
    bson::{self, doc, oid::ObjectId, Document},
    options::{FindOneAndUpdateOptions, ReturnDocument},
    Collection,
};

use crate::{
    shared::ApiError,
};

use super::TrackJson;

pub struct TrackManager {
    collection: Collection,
    playlist_id: ObjectId,
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
