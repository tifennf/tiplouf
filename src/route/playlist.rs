mod database;
mod error;
mod handler;
mod middleware;
mod schema;
mod utils;

use actix_web::web;

// /playlist/
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

    cfg.service(web::resource("/{id}/track/").route(web::post().to(handler::add_track)));

    cfg.service(
        web::resource("/{id}/track/{track_id}/")
            .route(web::delete().to(handler::remove_track))
            .route(web::get().to(handler::get_track)),
    );
}
