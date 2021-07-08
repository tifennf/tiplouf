use serde::Serialize;

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
