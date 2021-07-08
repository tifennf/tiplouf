use crate::{
    global::ApiResponse,
    route::playlist::{
        database::{PlaylistManager, TrackManager},
        schema,
    },
};
use actix_web::{web, HttpResponse, Result};
use mongodb::{
    bson::{self, oid::ObjectId},
    Client,
};

const DB: &str = "tiplouf";
const COLLECTION: &str = "playlist";

//get /playlist/
pub async fn get_all(client: web::Data<Client>) -> Result<HttpResponse> {
    let manager = PlaylistManager::init(client.database(DB).collection(COLLECTION));

    let result = manager.get_all().await;

    let res = match result {
        Ok(playlist_vec) => HttpResponse::Ok().json(ApiResponse::success(playlist_vec)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::fail(e.labels())),
    };

    Ok(res)
}

//post /playlist/
pub async fn create_one(
    client: web::Data<Client>,
    req_body: web::Json<schema::PlaylistRequest>,
) -> Result<HttpResponse> {
    let manager = PlaylistManager::init(client.database(DB).collection(COLLECTION));
    let playlist = req_body.0.draft();

    let result = manager.create_one(playlist).await;

    let res = match result {
        Ok(playlist) => HttpResponse::Ok().json(ApiResponse::success(playlist)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::fail(e.labels())),
    };

    Ok(res)
}

//delete /playlist/{id}
pub async fn delete_one(
    client: web::Data<Client>,
    web::Path(id): web::Path<String>,
) -> Result<HttpResponse> {
    let id = ObjectId::with_string(&id).unwrap();
    let manager = PlaylistManager::init(client.database(DB).collection(COLLECTION));

    let result = manager.delete_one(id).await;
    let res = match result {
        Ok(playlist) => HttpResponse::Ok().json(ApiResponse::success(playlist)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::fail(e.labels())),
    };

    Ok(res)
}

//get /playlist/{id}
pub async fn get_one(
    client: web::Data<Client>,
    web::Path(id): web::Path<String>,
) -> Result<HttpResponse> {
    let id = ObjectId::with_string(&id).unwrap();
    let manager = PlaylistManager::init(client.database(DB).collection(COLLECTION));

    let result = manager.get_one(id).await;

    let res = match result {
        Ok(playlist) => HttpResponse::Ok().json(ApiResponse::success(playlist)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::fail(e.labels())),
    };

    Ok(res)
}

//post /playlist/{id}/track
pub async fn add_track(
    client: web::Data<Client>,
    web::Path(id): web::Path<String>,
    req_body: web::Json<schema::TrackRequest>,
) -> Result<HttpResponse> {
    let id = ObjectId::with_string(&id).unwrap();
    let track = bson::to_document(&req_body.0.complete()).unwrap();
    let manager = TrackManager::init(client.database(DB).collection(COLLECTION), id);

    //add track inside tracklist field
    let result = manager.add_one(track).await;

    let res = match result {
        Ok(playlist) => HttpResponse::Ok().json(ApiResponse::fail(playlist)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::fail(e.labels())),
    };

    Ok(res)
}

pub async fn remove_track(
    client: web::Data<Client>,
    web::Path((id, track_id)): web::Path<(String, String)>,
) -> Result<HttpResponse> {
    let id = ObjectId::with_string(&id).unwrap();
    let track_id = ObjectId::with_string(&track_id).unwrap();
    let manager = TrackManager::init(client.database(DB).collection(COLLECTION), id);

    let result = manager.remove_one(track_id).await;

    let res = match result {
        Ok(playlist) => HttpResponse::Ok().json(ApiResponse::success(playlist)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::fail(e.labels())),
    };

    Ok(res)
}

pub async fn get_track(
    client: web::Data<Client>,
    web::Path((id, track_id)): web::Path<(String, String)>,
) -> Result<HttpResponse> {
    let id = ObjectId::with_string(&id).unwrap();
    let track_id = ObjectId::with_string(&track_id).unwrap();
    let manager = TrackManager::init(client.database(DB).collection(COLLECTION), id);

    let result = manager.get_one(track_id).await;

    let res = match result {
        Ok(playlist) => HttpResponse::Ok().json(ApiResponse::success(playlist)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::fail(e.labels())),
    };

    Ok(res)
}
