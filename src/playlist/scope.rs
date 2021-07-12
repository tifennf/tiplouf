use actix_web::web;

use super::handler as playlist;

pub enum Ressource {
    Playlist,
    Track,
}

// /playlist/
pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::get().to(playlist::get_all))
            .route(web::post().to(playlist::create_one)),
    );

    cfg.service(
        web::resource("/{id}/")
            .route(web::get().to(playlist::get_one))
            .route(web::delete().to(playlist::delete_one)),
    );

    // cfg.service(web::resource("/{id}/track/").route(web::post().to(track::create_track)));

    // cfg.service(
    //     web::resource("/{id}/track/{track_id}/")
    //         .route(web::delete().to(track::delete_track))
    //         .route(web::get().to(track::get_track)),
    // );
}
