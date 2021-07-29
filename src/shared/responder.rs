use actix_web::{dev::HttpResponseBuilder, http::StatusCode, HttpResponse};
use serde::Serialize;

use super::ApiError;

#[derive(Serialize)]
pub struct ApiResponse;

impl ApiResponse {
    pub fn success<T: Serialize>(data: T, status_code: StatusCode) -> HttpResponse {
        HttpResponseBuilder::new(status_code).json(ApiSuccess {
            status: "success".to_string(),
            data,
        })
    }

    pub fn fail(data: Option<ApiError>, status_code: StatusCode) -> HttpResponse {
        HttpResponseBuilder::new(status_code).json(ApiFail {
            status: "fail".to_string(),
            data,
        })
    }
}

#[derive(Serialize)]
pub struct ApiSuccess<T> {
    status: String,
    data: T,
}

impl ApiSuccess<String> {
    pub fn default() -> ApiSuccess<String> {
        ApiSuccess {
            status: "success".to_string(),
            data: "".to_string(),
        }
    }

}

#[derive(Serialize)]
struct ApiFail {
    status: String,
    data: Option<ApiError>,
}
