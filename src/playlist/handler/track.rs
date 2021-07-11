use crate::{
    playlist::{database::TrackManager, schema},
    shared::ApiResponse,
};
use actix_web::{http::StatusCode, web, HttpResponse, Result};
use mongodb::Client;

use super::{utils, COLLECTION, DB};

// POST /playlist/{p_id}/track
pub async fn add_track(
    client: web::Data<Client>,
    web::Path(id): web::Path<String>,
    body: web::Json<schema::TrackRequest>,
) -> Result<HttpResponse> {
    let (id, track) = (
        utils::validate_t_id(id)?,
        utils::validate_track(body.0)?,
    );

    let manager = TrackManager::init(client.database(DB).collection(COLLECTION), id);
    let playlist = manager.add_one(track).await?;

    Ok(ApiResponse::success(Some(playlist), StatusCode::CREATED))
}

// DELETE /playlist/{p_id}/track/{t_id}
pub async fn remove_track(
    client: web::Data<Client>,
    web::Path((p_id, t_id)): web::Path<(String, String)>,
) -> Result<HttpResponse> {
    let (p_id, t_id) = utils::validate_id_playlist_track(p_id, t_id)?;

    let manager = TrackManager::init(client.database(DB).collection(COLLECTION), p_id);
    let playlist = manager.remove_one(t_id).await?;

    Ok(ApiResponse::success(Some(playlist), StatusCode::ACCEPTED))
}

// GET /playlist/{p_id}/track/{t_id}
pub async fn get_track(
    client: web::Data<Client>,
    web::Path((p_id, t_id)): web::Path<(String, String)>,
) -> Result<HttpResponse> {
    let (p_id, t_id) = utils::validate_id_playlist_track(p_id, t_id)?;
    let manager = TrackManager::init(client.database(DB).collection(COLLECTION), p_id);

    let track = manager.get_one(t_id).await?;

    Ok(ApiResponse::success(Some(track), StatusCode::OK))
}
