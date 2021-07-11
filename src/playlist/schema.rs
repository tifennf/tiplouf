use std::collections::HashSet;

use crate::playlist::database as db;
use mongodb::bson::{doc, oid::ObjectId, Document};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaylistRequest {
    pub tracklist: HashSet<String>,
    pub tag: Option<String>,
}

impl PlaylistRequest {
    pub fn draft(self) -> db::PlaylistDraft {
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

        db::PlaylistDraft {
            tracklist,
            trackcount,
            tag: self.tag,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TrackRequest {
    url: String,
}

impl TrackRequest {
    pub fn complete(self) -> db::Track {
        let track_id = ObjectId::new();

        db::Track {
            url: self.url,
            track_id,
        }
    }
}
