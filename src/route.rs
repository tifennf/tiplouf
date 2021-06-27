mod playlist;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    // cfg.service(web::resource("/").route(web::get().to(index)));

    cfg.service(web::scope("/playlist/").configure(playlist_scope));

    cfg.service(web::scope("/test/"));
}

// /playlist/
fn playlist_scope(cfg: &mut web::ServiceConfig) {
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

    cfg.service(web::resource("/{id}/track/").route(web::post().to(playlist::add_track)));

    cfg.service(
        web::resource("/{id}/track/{track_id}/")
            .route(web::delete().to(playlist::remove_track))
            .route(web::get().to(playlist::get_track)),
    );
}
