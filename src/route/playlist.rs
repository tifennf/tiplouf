use crate::{
    route::utils,
    schema::{
        playlist::{Playlist, PlaylistJson, PlaylistRequest, Track, TrackRequest},
        ApiResponse,
    },
};
use actix_web::{web, HttpResponse, Responder};
use futures::StreamExt;
use mongodb::{
    bson::{doc, from_document, oid::ObjectId, to_document, Document},
    error::Error,
    options::{FindOneAndUpdateOptions, ReturnDocument},
    results::UpdateResult,
    Client,
};
use serde_json::Value;

const DB: &str = "tiplouf";
const COLLECTION: &str = "playlist";

//get /playlist/
pub async fn get_all(client: web::Data<Client>) -> impl Responder {
    let collection = client.database(DB).collection(COLLECTION);
    let collection = collection.find(None, None).await.unwrap();

    let collection = collection.collect::<Vec<Result<Document, Error>>>().await;

    let collection: Vec<PlaylistJson> = collection
        .iter()
        .map(|document| {
            let document = document.clone().unwrap();

            from_document::<Playlist>(document).unwrap().get_json()
        })
        .collect();

    HttpResponse::Ok().json(collection)
}

//post /playlist/
pub async fn create_one(
    client: web::Data<Client>,
    req_body: web::Json<PlaylistRequest>,
) -> impl Responder {
    let collection = client.database(DB).collection(COLLECTION);
    let playlist = req_body.0.draft();

    let id = collection
        .insert_one(playlist.get_doc(), None)
        .await
        .unwrap()
        .inserted_id;
    let id = id.as_object_id().unwrap().to_string();

    let response = ApiResponse {
        status: "success",
        data: playlist.get_json(id),
    };

    HttpResponse::Ok().json(response)
}

pub async fn delete_one(
    client: web::Data<Client>,
    web::Path(id): web::Path<String>,
) -> impl Responder {
    let playlist_collection = client.database(DB).collection(COLLECTION);

    let id = ObjectId::with_string(&id).unwrap();

    let deleted_playlist = playlist_collection
        .find_one_and_delete(doc! { "_id": id}, None)
        .await
        .unwrap();

    let response = ApiResponse {
        status: "success",
        data: from_document::<Playlist>(deleted_playlist.unwrap())
            .unwrap()
            .get_json(),
    };

    HttpResponse::Ok().json(response)
}

pub async fn get_one(
    client: web::Data<Client>,
    web::Path(id): web::Path<String>,
) -> impl Responder {
    let collection = client.database(DB).collection(COLLECTION);
    let id = ObjectId::with_string(&id).unwrap();

    let playlist = collection.find_one(doc! { "_id": id}, None).await.unwrap();

    let response = ApiResponse {
        status: "success",
        data: from_document::<Playlist>(playlist.unwrap())
            .unwrap()
            .get_json(),
    };

    HttpResponse::Ok().json(response)
}

pub async fn add_track(
    client: web::Data<Client>,
    web::Path(id): web::Path<String>,
    req_body: web::Json<TrackRequest>,
) -> impl Responder {
    let collection = client.database(DB).collection(COLLECTION);
    let id = ObjectId::with_string(&id).unwrap();
    let track = to_document(&req_body.0.complete()).unwrap();

    //add track inside tracklist field
    let add_track = doc! {"$push": {"tracklist": track}, "$inc": {"trackcount": 1}};
    let filter = doc! {"_id": id};

    let result = collection
        .update_one(filter.clone(), add_track, None)
        .await
        .unwrap();

    utils::parse_playlist_update(result, collection, filter).await
}

pub async fn remove_track(
    client: web::Data<Client>,
    web::Path((id, track_id)): web::Path<(String, String)>,
) -> impl Responder {
    let collection = client.database(DB).collection(COLLECTION);
    let id = ObjectId::with_string(&id).unwrap();
    let track_id = ObjectId::with_string(&track_id).unwrap();

    //delete track inside tracklist field
    let filter = doc! {"_id": id};
    let remove_track = doc! {"$pull": {"tracklist": { "track_id": {"$eq": track_id}}}};

    let result = collection
        .update_one(filter.clone(), remove_track, None)
        .await
        .unwrap();

    utils::parse_playlist_update(result, collection, filter).await
}
