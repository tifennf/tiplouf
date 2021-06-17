use std::{collections::HashSet, usize};

use mongodb::bson::{doc, from_document, oid::ObjectId, to_document, Document};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaylistRequest {
    pub tracklist: HashSet<String>,
    pub tag: Option<String>,
}

impl PlaylistRequest {
    pub fn draft(self) -> PlaylistDraft {
        let tracklist = self
            .tracklist
            .iter()
            .map(|track| {
                let track_id = ObjectId::new();
                doc! {
                    "url": track,
                    "track_id": track_id,
                }
            })
            .collect::<Vec<Document>>();
        let trackcount = tracklist.len() as i64;

        PlaylistDraft {
            tracklist,
            trackcount,
            tag: self.tag,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PlaylistDraft {
    tracklist: Vec<Document>,
    trackcount: i64,
    tag: Option<String>,
}

impl PlaylistDraft {
    pub fn get_doc(&self) -> Document {
        to_document(self).unwrap()
    }

    pub fn get_json(&self, id: String) -> PlaylistJson {
        let tracklist = self
            .tracklist
            .iter()
            .map(|track| from_document::<Track>(track.clone()).unwrap().get_json())
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
    tracklist: Vec<Track>,
    trackcount: i64,
    tag: Option<String>,
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
    tracklist: Vec<TrackJson>,
    trackcount: i64,
    tag: Option<String>,
    id: String,
}

#[derive(Serialize, Deserialize)]
pub struct Track {
    url: String,
    track_id: ObjectId,
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
    url: String,
    track_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct TrackRequest {
    url: String,
}

impl TrackRequest {
    pub fn complete(self) -> Track {
        let track_id = ObjectId::new();

        Track {
            url: self.url,
            track_id,
        }
    }
}
