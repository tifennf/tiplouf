use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

use crate::track::schema::TrackJson;
pub struct TrackDraft {

}

#[derive(Serialize, Deserialize)]
pub struct Track {
    pub url: String,
    pub id: ObjectId,
}

impl Track {
    pub fn to_json(self) -> TrackJson {
        TrackJson {
            url: self.url,
            id: self.id.to_string(),
        }
    }
}
