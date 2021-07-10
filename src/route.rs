pub mod playlist;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/playlist/").configure(playlist::scope));
}
