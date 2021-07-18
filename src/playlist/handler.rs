use std::collections::HashSet;

use crate::{
    playlist::{self, database::PlaylistManager},
    shared::{utils, ApiError, ApiResponse},
    track::database::TrackDraft,
};
use actix_web::{http::StatusCode, web, HttpResponse, Result};
use mongodb::Database;

//get /playlist/
pub async fn get_all(database: web::Data<Database>) -> Result<HttpResponse> {
    let manager = PlaylistManager::init(&database);
    let p_list = manager.get_all().await?;

    Ok(ApiResponse::success(Some(p_list), StatusCode::OK))
}

//get /playlist/{id}
pub async fn get_one(
    database: web::Data<Database>,
    web::Path(id): web::Path<String>,
) -> Result<HttpResponse> {
    let id = utils::validate_p_id(&id)?;

    let manager = PlaylistManager::init(&database);
    let playlist = manager.get_one(id).await?;

    Ok(ApiResponse::success(playlist, StatusCode::OK))
}

//post /playlist/
pub async fn create_one(
    database: web::Data<Database>,
    body: web::Json<playlist::PlaylistRequest>,
) -> Result<HttpResponse> {
    let playlist = body.0;

    let manager = PlaylistManager::init(&database);
    let playlist = manager.add_one(playlist).await?;

    Ok(ApiResponse::success(Some(playlist), StatusCode::CREATED))
}

//delete /playlist/{id}
pub async fn delete_one(
    database: web::Data<Database>,
    web::Path(id): web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let id = utils::validate_p_id(&id)?;

    let manager = PlaylistManager::init(&database);
    let playlist = manager.remove_one(id).await?;

    Ok(ApiResponse::success(Some(playlist), StatusCode::ACCEPTED))
}

pub async fn tracklist_add(
    database: web::Data<Database>,
    web::Path(id): web::Path<String>,
    body: web::Json<HashSet<String>>,
) -> Result<HttpResponse> {
    let id = utils::validate_p_id(&id)?;
    let tracklist = body
        .0
        .into_iter()
        .map(|url| TrackDraft::new(url, id.clone()))
        .collect();

    let manager = PlaylistManager::init(&database);

    let playlist = manager.add_track(id, tracklist).await?;

    Ok(ApiResponse::success(Some(playlist), StatusCode::CREATED))
}
pub async fn tracklist_remove(
    database: web::Data<Database>,
    web::Path(id): web::Path<String>,
    body: web::Json<HashSet<String>>,
) -> Result<HttpResponse> {
    let id = utils::validate_p_id(&id)?;
    let id_list = body.0;
    let manager = PlaylistManager::init(&database);

    let playlist = manager.remove_track(id, id_list).await?;

    Ok(ApiResponse::success(Some(playlist), StatusCode::ACCEPTED))
}
