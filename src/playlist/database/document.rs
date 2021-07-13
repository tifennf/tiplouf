use mongodb::{Collection, Database, bson::{self, doc, oid::ObjectId, Document}};
use serde::{Deserialize, Serialize};

use crate::{playlist::schema::PlaylistJson, shared::ApiError, track::{self, TrackJson, TrackManager}};

#[derive(Serialize, Deserialize)]
pub struct PlaylistDraft {
    pub tracklist: Vec<Document>,
    pub trackcount: i64,
    pub tag: Option<String>,
}

// impl PlaylistDraft {
//     pub async fn to_json(self, id: String, database: &Database) -> Result<PlaylistJson, ApiError> {
//         let tracklist = map_tracklist(self.tracklist, database).await?;

//         let playlist = PlaylistJson {
//             tracklist: self.tracklist,
//             trackcount: self.trackcount,
//             tag: self.tag,
//             id,
//         };

//         Ok(playlist)
//     }
// }

#[derive(Serialize, Deserialize)]
struct TrackRef {
    #[serde(rename = "_id")]
    pub id: ObjectId,
}

#[derive(Serialize, Deserialize)]
pub struct Playlist {
    pub tracklist: Vec<TrackRef>,
    pub trackcount: i64,
    pub tag: Option<String>,
    #[serde(rename = "_id")]
    pub id: ObjectId,
}

impl Playlist {
    pub async fn to_json(self, collection: &Collection) -> Result<PlaylistJson, ApiError> {
        
        let tracklist = map_tracklist(self.tracklist, collection).await?;
        
        let playlist = PlaylistJson {
            tracklist,
            trackcount: self.trackcount,
            tag: self.tag,
            id: self.id.to_string(),
        };

        Ok(playlist)
    }
}

async fn map_tracklist(tracklist: Vec<TrackRef>, t_collection: &Collection) -> Result<Vec<Option<TrackJson>>, ApiError> {
    let manager = TrackManager::init(t_collection.clone()).await;
    let mut t_list = Vec::new();
    for track in tracklist.iter() {
        t_list.push(manager.get_one(track.id.clone()).await?);
    }

    Ok(t_list)
}

