use mongodb::bson::{self, doc, oid::ObjectId, Document};
use serde::{Deserialize, Serialize};

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
