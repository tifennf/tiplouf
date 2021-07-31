use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

use crate::track::schema::TrackJson;
#[derive(Serialize, Deserialize, Clone)]
pub struct TrackDraft {
    pub url: String,
    pub p_id: ObjectId,
}

impl TrackDraft {
    pub fn new(url: String, p_id: ObjectId) -> TrackDraft {
        TrackDraft { url, p_id }
    }

    pub fn into_json(self) -> TrackJson {
        let p_id = self.p_id.to_string();

        TrackJson {
            url: self.url,
            p_id,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Track {
    pub url: String,
    pub p_id: ObjectId,
    #[serde(rename = "_id")]
    pub id: ObjectId,
}

impl Track {
    pub fn into_json(self) -> TrackJson {
        let p_id = self.p_id.to_string();

        TrackJson {
            url: self.url,
            p_id,
        }
    }
}
