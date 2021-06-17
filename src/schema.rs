use serde::{Deserialize, Serialize};

pub mod playlist;

#[derive(Serialize)]
pub struct ApiResponse<'a, T> {
    pub status: &'a str,
    pub data: T,
}

impl<'a, T> ApiResponse<'a, T> {
    pub fn success(data: T) -> ApiResponse<'a, T> {
        ApiResponse {
            status: "success",
            data,
        }
    }

    pub fn fail(data: T) -> ApiResponse<'a, T> {
        ApiResponse {
            status: "fail",
            data,
        }
    }
}
