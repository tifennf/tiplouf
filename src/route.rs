use crate::{playlist, shared::middleware::SayHi, user};

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth/").configure(user::scope));
    cfg.service(web::scope("/playlist/").configure(playlist::scope).wrap(SayHi));

}
