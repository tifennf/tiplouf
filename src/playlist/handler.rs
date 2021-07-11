use crate::{
    playlist::{
        self,
        database::{PlaylistManager, TrackManager},
        schema,
    },
    shared::{ApiError, ApiResponse},
};
use actix_web::{http::StatusCode, web, HttpResponse, Result};
use mongodb::{
    bson::{self, oid::ObjectId},
    Client,
};

const DB: &str = "tiplouf";
const COLLECTION: &str = "playlist";

//get /playlist/
pub async fn get_all(client: web::Data<Client>) -> Result<HttpResponse> {
    let manager = PlaylistManager::init(client.database(DB).collection(COLLECTION));
    let p_list = manager.get_all().await?;

    Ok(ApiResponse::success(Some(p_list), StatusCode::OK))
}

//post /playlist/
pub async fn create_one(
    client: web::Data<Client>,
    req_body: web::Json<schema::PlaylistRequest>,
) -> Result<HttpResponse> {
    let playlist = req_body.0.draft();

    let manager = PlaylistManager::init(client.database(DB).collection(COLLECTION));
    let playlist = manager.create_one(playlist).await?;

    Ok(ApiResponse::success(Some(playlist), StatusCode::CREATED))
}

//delete /playlist/{id}
pub async fn delete_one(
    client: web::Data<Client>,
    web::Path(p_id): web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let id = validate_p_id(p_id)?;

    let manager = PlaylistManager::init(client.database(DB).collection(COLLECTION));
    let playlist = manager.delete_one(id).await?;

    Ok(ApiResponse::success(Some(playlist), StatusCode::ACCEPTED))
}

//get /playlist/{id}
pub async fn get_one(
    client: web::Data<Client>,
    web::Path(p_id): web::Path<String>,
) -> Result<HttpResponse> {
    let p_id = validate_p_id(p_id)?;

    let manager = PlaylistManager::init(client.database(DB).collection(COLLECTION));
    let playlist = manager.get_one(p_id).await?;

    Ok(ApiResponse::success(Some(playlist), StatusCode::OK))
}

// /playlist/{id}/track
pub async fn add_track(
    client: web::Data<Client>,
    web::Path(id): web::Path<String>,
    req_body: web::Json<schema::TrackRequest>,
) -> Result<HttpResponse, ApiError> {
    let (id, track) = (
        validate_track_id(id)?,
        bson::to_document(&req_body.0.complete())?,
    );

    let manager = TrackManager::init(client.database(DB).collection(COLLECTION), id);
    let playlist = manager.add_one(track).await?;

    Ok(ApiResponse::success(Some(playlist), StatusCode::CREATED))
}

pub async fn remove_track(
    client: web::Data<Client>,
    web::Path((p_id, track_id)): web::Path<(String, String)>,
) -> Result<HttpResponse> {
    let (p_id, track_id) = validate_id_playlist_track(p_id, track_id)?;

    let manager = TrackManager::init(client.database(DB).collection(COLLECTION), p_id);
    let playlist = manager.remove_one(track_id).await?;

    Ok(ApiResponse::success(Some(playlist), StatusCode::ACCEPTED))
}

pub async fn get_track(
    client: web::Data<Client>,
    web::Path((p_id, track_id)): web::Path<(String, String)>,
) -> Result<HttpResponse> {
    let (p_id, track_id) = validate_id_playlist_track(p_id, track_id)?;
    let manager = TrackManager::init(client.database(DB).collection(COLLECTION), p_id);

    let track = manager.get_one(track_id).await?;

    Ok(ApiResponse::success(Some(track), StatusCode::OK))
}

// fn respond_playlist(playlist: PlaylistJson, code: StatusCode) -> ApiSuccess {
//     ApiResponse::success(Some(playlist), code)
// }

fn validate_p_id(p_id: String) -> Result<ObjectId, ApiError> {
    ObjectId::with_string(&p_id).map_err(|_| ApiError::ValidationError {
        info: playlist::error::Playlist::InvalidId.to_string(),
    })
}

fn validate_track_id(track_id: String) -> Result<ObjectId, ApiError> {
    ObjectId::with_string(&track_id).map_err(|_| ApiError::ValidationError {
        info: playlist::error::Playlist::TrackInvalidId.to_string(),
    })
}

fn validate_id_playlist_track(
    p_id: String,
    track_id: String,
) -> Result<(ObjectId, ObjectId), ApiError> {
    let p_id = validate_p_id(p_id)?;
    let track_id = validate_track_id(track_id)?;

    Ok((p_id, track_id))
}
