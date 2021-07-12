const DB: &str = "tiplouf";
const COLLECTION: &str = "playlist";

use crate::{playlist::{database::PlaylistManager, self}, shared::{ApiError, ApiResponse, utils}, track::{self, database::TrackManager}};
use actix_web::{http::StatusCode, web, HttpResponse, Result};
use mongodb::Client;

//get /playlist/
pub async fn get_all(client: web::Data<Client>) -> Result<HttpResponse> {
    let manager = PlaylistManager::init(client.database(DB).collection(COLLECTION));
    let p_list = manager.get_all().await?;
    
    Ok(ApiResponse::success(Some(p_list), StatusCode::OK))
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

//post /playlist/
pub async fn create_one(
    client: web::Data<Client>,
    body: web::Json<playlist::PlaylistRequest>,
) -> Result<HttpResponse> {
    let playlist = body.0.draft();

    let manager = PlaylistManager::init(client.database(DB).collection(COLLECTION));
    let playlist = manager.add_one(playlist).await?;

    Ok(ApiResponse::success(Some(playlist), StatusCode::CREATED))
}

//delete /playlist/{id}
pub async fn delete_one(
    client: web::Data<Client>,
    web::Path(id): web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let id = utils::validate_p_id(id)?;

    let manager = PlaylistManager::init(client.database(DB).collection(COLLECTION));
    let playlist = manager.remove_one(id).await?;

    Ok(ApiResponse::success(Some(playlist), StatusCode::ACCEPTED))
}


// // GET /playlist/{p_id}/track/{t_id}
// pub async fn get_track(
//     client: web::Data<Client>,
//     web::Path((p_id, t_id)): web::Path<(String, String)>,
// ) -> Result<HttpResponse> {
//     let (p_id, t_id) = utils::validate_id_playlist_track(p_id, t_id)?;
//     let manager = TrackManager::init(client.database(DB).collection(COLLECTION), p_id);

//     let track = manager.get_one(t_id).await?;

//     Ok(ApiResponse::success(Some(track), StatusCode::OK))
// }

// // POST /playlist/{p_id}/track
// pub async fn create_track(
//     client: web::Data<Client>,
//     web::Path(id): web::Path<String>,
//     body: web::Json<track::TrackRequest>,
// ) -> Result<HttpResponse> {
//     let (id, track) = (
//         utils::validate_t_id(id)?,
//         utils::validate_track(body.0)?,
//     );

//     let manager = TrackManager::init(client.database(DB).collection(COLLECTION), id);
//     let playlist = manager.add_one(track).await?;

//     Ok(ApiResponse::success(Some(playlist), StatusCode::CREATED))
// }

// // DELETE /playlist/{p_id}/track/{t_id}
// pub async fn delete_track(
//     client: web::Data<Client>,
//     web::Path((p_id, t_id)): web::Path<(String, String)>,
// ) -> Result<HttpResponse> {
//     let (p_id, t_id) = utils::validate_id_playlist_track(p_id, t_id)?;

//     let manager = TrackManager::init(client.database(DB).collection(COLLECTION), p_id);
//     let playlist = manager.remove_one(t_id).await?;

//     Ok(ApiResponse::success(Some(playlist), StatusCode::ACCEPTED))
// }
