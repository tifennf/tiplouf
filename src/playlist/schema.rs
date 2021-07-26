use std::collections::HashSet;

use crate::track::TrackJson;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

use super::database::PlaylistDraft;

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaylistRequest {
    pub tracklist: HashSet<String>,
    pub tag: Option<String>,
}

impl PlaylistRequest {
    pub fn into_draft(self) -> (HashSet<String>, PlaylistDraft) {
        let draft = PlaylistDraft { tag: self.tag };

        (self.tracklist, draft)
    }
}

#[derive(Serialize, Deserialize)]
pub struct PlaylistJson {
    pub tracklist: Vec<TrackJson>,
    pub tag: Option<String>,
    pub id: String,
}

#[derive(Deserialize)]
pub struct Info {
    pub tag: String,
}
