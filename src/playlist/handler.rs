use std::collections::HashSet;

use crate::{
    playlist::{self, database::PlaylistManager},
    shared::{utils, ApiError, ApiResponse},
    track::database::TrackDraft,
};
use actix_web::{
    http::StatusCode,
    web::{self, Query},
    HttpRequest, HttpResponse, Result,
};
use mongodb::Database;

use super::schema::Info;

//get /playlist/
pub async fn get_all(database: web::Data<Database>, req: HttpRequest) -> Result<HttpResponse> {
    let user_id = utils::extract_user_id(&req)?;

    let manager = PlaylistManager::new(&database);
    let p_list = manager.get_all(user_id).await?;

    Ok(ApiResponse::success(p_list, StatusCode::OK))
}

pub async fn tag_get_all(
    database: web::Data<Database>,
    req: HttpRequest,
    query: Query<Info>,
) -> Result<HttpResponse> {
    let user_id = utils::extract_user_id(&req)?;

    let manager = PlaylistManager::new(&database);
    let p_list = manager.get_tag(user_id, query.tag.clone()).await?;

    Ok(ApiResponse::success(p_list, StatusCode::OK))
}

//get /playlist/{id}
pub async fn get_one(
    database: web::Data<Database>,
    req: HttpRequest,
    web::Path(p_id): web::Path<String>,
) -> Result<HttpResponse> {
    let user_id = utils::extract_user_id(&req)?;
    let p_id = utils::validate_p_id(&p_id)?;

    let manager = PlaylistManager::new(&database);
    let playlist = manager.get_one(user_id, p_id).await?;

    Ok(ApiResponse::success(playlist, StatusCode::OK))
}

//post /playlist/
pub async fn create_one(
    database: web::Data<Database>,
    req: HttpRequest,
    body: web::Json<playlist::PlaylistRequest>,
) -> Result<HttpResponse> {
    let user_id = utils::extract_user_id(&req)?;
    let playlist = body.0;

    let manager = PlaylistManager::new(&database);

    let playlist = manager.add_one(user_id, playlist).await?;

    Ok(ApiResponse::success(playlist, StatusCode::CREATED))
}

//delete /playlist/{id}
pub async fn delete_one(
    database: web::Data<Database>,
    req: HttpRequest,
    web::Path(p_id): web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let user_id = utils::extract_user_id(&req)?;
    let p_id = utils::validate_p_id(&p_id)?;

    let manager = PlaylistManager::new(&database);
    let playlist = manager.remove_one(user_id, p_id).await?;

    Ok(ApiResponse::success(playlist, StatusCode::ACCEPTED))
}

pub async fn tracklist_add(
    database: web::Data<Database>,
    req: HttpRequest,
    web::Path(p_id): web::Path<String>,
    body: web::Json<HashSet<String>>,
) -> Result<HttpResponse> {
    let user_id = utils::extract_user_id(&req)?;
    let p_id = utils::validate_p_id(&p_id)?;
    let tracklist = body
        .0
        .into_iter()
        .map(|url| TrackDraft::new(url, p_id.clone()))
        .collect();

    let manager = PlaylistManager::new(&database);

    let playlist = manager.add_track(user_id, p_id, tracklist).await?;

    Ok(ApiResponse::success(playlist, StatusCode::CREATED))
}
pub async fn tracklist_remove(
    database: web::Data<Database>,
    req: HttpRequest,
    web::Path(p_id): web::Path<String>,
    body: web::Json<HashSet<String>>,
) -> Result<HttpResponse> {
    let user_id = utils::extract_user_id(&req)?;
    let p_id = utils::validate_p_id(&p_id)?;
    let id_list = body.0;
    let manager = PlaylistManager::new(&database);

    let playlist = manager.remove_track(user_id, p_id, id_list).await?;

    Ok(ApiResponse::success(playlist, StatusCode::ACCEPTED))
}
