use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::{playlist::schema::PlaylistJson, track::TrackJson};

#[derive(Serialize, Deserialize)]
pub struct PlaylistDraft {
    pub tag: Option<String>,
    pub user_id: ObjectId,
}

impl PlaylistDraft {
    pub fn into_json(self, tracklist: Vec<TrackJson>, id: &ObjectId) -> PlaylistJson {
        let id = id.to_string();

        PlaylistJson {
            tracklist,
            tag: self.tag,
            id,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct TrackRef {
    #[serde(rename = "_id")]
    pub id: ObjectId,
}

#[derive(Serialize, Deserialize)]
pub struct Playlist {
    pub tag: Option<String>,
    #[serde(rename = "_id")]
    pub id: ObjectId,
}

impl Playlist {
    pub fn into_json(self, tracklist: Vec<TrackJson>) -> PlaylistJson {
        PlaylistJson {
            tracklist,
            tag: self.tag,
            id: self.id.to_string(),
        }
    }
}
