use actix_web::web;

use super::handler;

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/register/").route(web::post().to(handler::register)));
    cfg.service(web::resource("/login/").route(web::post().to(handler::login)));
}
