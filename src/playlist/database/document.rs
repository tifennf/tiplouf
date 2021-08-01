use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::{playlist::schema::PlaylistJson, track::TrackJson};

#[derive(Serialize, Deserialize)]
pub struct PlaylistDraft {
    pub tag: Option<String>,
    pub user_id: ObjectId,
}

impl PlaylistDraft {
    pub fn into_json(self, tracklist: Vec<TrackJson>, p_id: &ObjectId) -> PlaylistJson {
        let p_id = p_id.to_string();

        PlaylistJson {
            tracklist,
            tag: self.tag,
            p_id,
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
    pub p_id: ObjectId,
}

impl Playlist {
    pub fn into_json(self, tracklist: Vec<TrackJson>) -> PlaylistJson {
        PlaylistJson {
            tracklist,
            tag: self.tag,
            p_id: self.p_id.to_string(),
        }
    }
}
