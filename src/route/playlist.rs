use crate::{
    database::playlist::{PlaylistManager, TrackManager},
    schema::{ApiResponse, PlaylistRequest, TrackRequest},
};
use actix_web::{web, HttpResponse, Responder};
use mongodb::{
    bson::{self, oid::ObjectId},
    Client,
};

const DB: &str = "tiplouf";
const COLLECTION: &str = "playlist";

//get /playlist/
pub async fn get_all(client: web::Data<Client>) -> impl Responder {
    let manager = PlaylistManager::init(client.database(DB).collection(COLLECTION));

    let res = manager.get_all().await;

    HttpResponse::Ok().json(ApiResponse::success(res))
}

//post /playlist/
pub async fn create_one(
    client: web::Data<Client>,
    req_body: web::Json<PlaylistRequest>,
) -> impl Responder {
    let manager = PlaylistManager::init(client.database(DB).collection(COLLECTION));
    let playlist = req_body.0.draft();

    let res = manager.create_one(playlist).await;

    HttpResponse::Ok().json(ApiResponse::success(res))
}

//delete /playlist/{id}
pub async fn delete_one(
    client: web::Data<Client>,
    web::Path(id): web::Path<String>,
) -> impl Responder {
    let id = ObjectId::with_string(&id).unwrap();
    let manager = PlaylistManager::init(client.database(DB).collection(COLLECTION));

    let res = ApiResponse {
        status: "success",
        data: manager.delete_one(id).await,
    };

    HttpResponse::Ok().json(res)
}

//get /playlist/{id}
pub async fn get_one(
    client: web::Data<Client>,
    web::Path(id): web::Path<String>,
) -> impl Responder {
    let id = ObjectId::with_string(&id).unwrap();
    let manager = PlaylistManager::init(client.database(DB).collection(COLLECTION));

    let playlist = manager.get_one(id).await;

    let response = ApiResponse {
        status: "success",
        data: playlist,
    };

    HttpResponse::Ok().json(response)
}

//post /playlist/{id}/track
pub async fn add_track(
    client: web::Data<Client>,
    web::Path(id): web::Path<String>,
    req_body: web::Json<TrackRequest>,
) -> impl Responder {
    let id = ObjectId::with_string(&id).unwrap();
    let track = bson::to_document(&req_body.0.complete()).unwrap();
    let manager = TrackManager::init(client.database(DB).collection(COLLECTION), id);

    //add track inside tracklist field
    let result = manager.add_one(track).await;

    match result {
        Ok(playlist) => HttpResponse::Ok().json(ApiResponse::fail(playlist)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::fail(e.labels())),
    }
}

pub async fn remove_track(
    client: web::Data<Client>,
    web::Path((id, track_id)): web::Path<(String, String)>,
) -> impl Responder {
    let id = ObjectId::with_string(&id).unwrap();
    let track_id = ObjectId::with_string(&track_id).unwrap();
    let manager = TrackManager::init(client.database(DB).collection(COLLECTION), id);

    let result = manager.remove_one(track_id).await;

    match result {
        Ok(playlist) => HttpResponse::Ok().json(ApiResponse::fail(playlist)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::fail(e.labels())),
    }
}
