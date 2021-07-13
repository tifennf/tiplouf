use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TrackRequest {
    url: String,
}

impl TrackRequest {
    pub fn to_json(self, id: String) -> TrackJson {
        TrackJson {
            url: self.url,
            id,
        }
    }
}


#[derive(Serialize, Deserialize)]
pub struct TrackJson {
    pub url: String,
    pub id: String,
}

impl Clone for TrackJson {
    fn clone(&self) -> Self {
        TrackJson {
            url: self.url.clone(),
            id: self.id.clone(),
        }
    }
}