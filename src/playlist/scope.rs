use actix_web::web;

use super::handler;

pub enum Ressource {
    Playlist,
    Track,
}

// /handler/
pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::get().to(handler::get_all))
            .route(web::post().to(handler::create_one)),
    );

    cfg.service(
        web::resource("/{id}/")
            .route(web::get().to(handler::get_one))
            .route(web::delete().to(handler::delete_one)),
    );

    cfg.service(web::resource("/{id}/track/").route(web::post().to(handler::tracklist_add)));

    cfg.service(
        web::resource("/{id}/track/{track_id}/").route(web::delete().to(handler::tracklist_remove)),
    );
}
