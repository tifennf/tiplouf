use crate::{
    playlist::{database::PlaylistManager, schema},
    shared::{ApiError, ApiResponse},
};
use actix_web::{http::StatusCode, web, HttpResponse, Result};
use mongodb::Client;

use super::{utils, COLLECTION, DB};

//get /playlist/
pub async fn get_all(client: web::Data<Client>) -> Result<HttpResponse> {
    let manager = PlaylistManager::init(client.database(DB).collection(COLLECTION));
    let p_list = manager.get_all().await?;

    Ok(ApiResponse::success(Some(p_list), StatusCode::OK))
}

//post /playlist/
pub async fn create_one(
    client: web::Data<Client>,
    body: web::Json<schema::PlaylistRequest>,
) -> Result<HttpResponse> {
    let playlist = body.0.draft();

    let manager = PlaylistManager::init(client.database(DB).collection(COLLECTION));
    let playlist = manager.create_one(playlist).await?;

    Ok(ApiResponse::success(Some(playlist), StatusCode::CREATED))
}

//delete /playlist/{id}
pub async fn delete_one(
    client: web::Data<Client>,
    web::Path(id): web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let id = utils::validate_p_id(id)?;

    let manager = PlaylistManager::init(client.database(DB).collection(COLLECTION));
    let playlist = manager.delete_one(id).await?;

    Ok(ApiResponse::success(Some(playlist), StatusCode::ACCEPTED))
}

//get /playlist/{id}
pub async fn get_one(
    client: web::Data<Client>,
    web::Path(id): web::Path<String>,
) -> Result<HttpResponse> {
    let id = utils::validate_p_id(id)?;

    let manager = PlaylistManager::init(client.database(DB).collection(COLLECTION));
    let playlist = manager.get_one(id).await?;

    Ok(ApiResponse::success(Some(playlist), StatusCode::OK))
}
