use actix_web::{HttpResponse, Responder};
use mongodb::{
    bson::{self, Document},
    results::UpdateResult,
    Collection,
};

use crate::schema::{playlist::Playlist, ApiResponse};

pub async fn parse_playlist_update(
    result: UpdateResult,
    collection: Collection,
    filter: Document,
) -> impl Responder {
    if result.matched_count != 1 || result.modified_count != 1 {
        //query ( id ) not found or track id not found

        let response = ApiResponse::fail("Not found");

        return HttpResponse::NotFound().json(response);
    }

    // collection.update_one(query, update, options)

    let playlist = collection.find_one(filter, None).await.unwrap().unwrap();

    let response = ApiResponse::success(
        bson::from_document::<Playlist>(playlist)
            .unwrap()
            .get_json(),
    );
    HttpResponse::Ok().json(response)
}
