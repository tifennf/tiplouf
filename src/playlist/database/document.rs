use mongodb::{Collection, Database, bson::{self, doc, oid::ObjectId, Document}};
use serde::{Deserialize, Serialize};

use crate::{playlist::schema::PlaylistJson, shared::ApiError, track::{self, TrackJson, TrackManager}};

#[derive(Serialize, Deserialize)]
pub struct PlaylistDraft {
    pub trackcount: i64,
    pub tag: Option<String>,
}

impl PlaylistDraft {
    pub fn to_json(self, tracklist: Vec<TrackJson>, id: &ObjectId) -> PlaylistJson {
        let id = id.to_string();

        let playlist = PlaylistJson {
            tracklist,
            trackcount: self.trackcount,
            tag: self.tag,
            id,
        };

        playlist
    }
}

#[derive(Serialize, Deserialize)]
struct TrackRef {
    #[serde(rename = "_id")]
    pub id: ObjectId,
}

#[derive(Serialize, Deserialize)]
pub struct Playlist {
    pub trackcount: i64,
    pub tag: Option<String>,
    #[serde(rename = "_id")]
    pub id: ObjectId,
}

impl Playlist {
    pub fn to_json(self, tracklist: Vec<TrackJson>) -> PlaylistJson {
        let playlist = PlaylistJson {
            tracklist,
            trackcount: self.trackcount,
            tag: self.tag,
            id: self.id.to_string(),
        };

        playlist
    }
}

// async fn map_tracklist(tracklist: Vec<TrackRef>, t_collection: &Collection) -> Result<Vec<Option<TrackJson>>, ApiError> {
//     let manager = TrackManager::init(t_collection.clone()).await;
//     let mut t_list = Vec::new();
//     for track in tracklist.iter() {
//         t_list.push(manager.get_one(track.id.clone()).await?);
//     }

//     Ok(t_list)
// }

