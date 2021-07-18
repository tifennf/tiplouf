use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::{playlist::schema::PlaylistJson, track::TrackJson};

#[derive(Serialize, Deserialize)]
pub struct PlaylistDraft {
    pub trackcount: i64,
    pub tag: Option<String>,
}

impl PlaylistDraft {
    pub fn into_json(self, tracklist: Vec<TrackJson>, id: &ObjectId) -> PlaylistJson {
        let id = id.to_string();

        PlaylistJson {
            tracklist,
            trackcount: self.trackcount,
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
    pub trackcount: i64,
    pub tag: Option<String>,
    #[serde(rename = "_id")]
    pub id: ObjectId,
}

impl Playlist {
    pub fn into_json(self, tracklist: Vec<TrackJson>) -> PlaylistJson {
        PlaylistJson {
            tracklist,
            trackcount: self.trackcount,
            tag: self.tag,
            id: self.id.to_string(),
        }
    }
}
