use std::collections::HashSet;

use mongodb::bson::{doc, oid::ObjectId, Document};
use serde::{Deserialize, Serialize};

use crate::database::playlist::{PlaylistDraft, Track};

#[derive(Serialize)]
pub struct ApiResponse {}

impl ApiResponse {
    pub fn success<T>(data: T) -> ApiSuccess<T> {
        ApiSuccess {
            status: "success".to_string(),
            data,
        }
    }

    pub fn fail<T>(data: T) -> ApiFail<T> {
        ApiFail {
            status: "fail".to_string(),
            data,
        }
    }
}

// #[derive(Serialize)]
// pub struct ApiResponse<T> {
//     status: String,
//     data: T,
// }

// impl<T> ApiResponse<T> {
//     pub fn success(data: T) -> ApiResponse<T> {
//         ApiResponse {
//             status: "success".to_string(),
//             data,
//         }
//     }

//     pub fn fail(data: T) -> ApiResponse<T> {
//         ApiResponse {
//             status: "fail".to_string(),
//             data,
//         }
//     }
// }

#[derive(Serialize)]
pub struct ApiSuccess<T> {
    status: String,
    data: T,
}

#[derive(Serialize)]
pub struct ApiFail<T> {
    status: String,
    data: T,
}

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
