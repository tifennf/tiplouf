use std::collections::HashSet;

use crate::{playlist::database as db, track::TrackJson};
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
                doc! {
                    "url": track,
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
pub struct PlaylistJson {
    pub tracklist: Vec<Option<TrackJson>>,
    pub trackcount: i64,
    pub tag: Option<String>,
    pub id: String,
}


