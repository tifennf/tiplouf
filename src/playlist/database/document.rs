use mongodb::bson::{self, doc, oid::ObjectId, Document};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PlaylistDraft {
    pub tracklist: Vec<Document>,
    pub trackcount: i64,
    pub tag: Option<String>,
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
    pub tracklist: Vec<Track>,
    pub trackcount: i64,
    pub tag: Option<String>,
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
    pub url: String,
    pub track_id: ObjectId,
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

impl Clone for TrackJson {
    fn clone(&self) -> Self {
        TrackJson {
            url: self.url.clone(),
            track_id: self.track_id.clone(),
        }
    }
}
