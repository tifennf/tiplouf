use std::collections::HashSet;
use super::database::Track;

use mongodb::bson::{doc, oid::ObjectId, Document};
use serde::{Deserialize, Serialize};

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