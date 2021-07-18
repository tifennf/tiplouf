use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

use super::database::TrackDraft;

#[derive(Serialize, Deserialize, Debug)]
pub struct TrackRequest {
    url: String,
}

impl TrackRequest {
    pub fn into_draft(self, p_id: ObjectId) -> TrackDraft {
        TrackDraft {
            url: self.url,
            p_id,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TrackJson {
    pub url: String,
    pub p_id: String,
    pub id: String,
}

impl Clone for TrackJson {
    fn clone(&self) -> Self {
        TrackJson {
            url: self.url.clone(),
            p_id: self.p_id.clone(),
            id: self.id.clone(),
        }
    }
}
