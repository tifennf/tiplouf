pub mod playlist;
pub mod route;
pub mod shared;
pub mod track;
pub mod user;

use std::sync::RwLock;

use actix_cors::Cors;
// use actix_files::Files;
use actix_web::{
    middleware::{Logger, NormalizePath},
    web, App, HttpServer,
};
use bimap::BiMap;
use mongodb::{bson::oid::ObjectId, Client};

const ADDR: &str = "0.0.0.0:5000";
const DB: &str = "tiplouf";

pub async fn start(client: Client) -> std::io::Result<()> {
    let session_list = RwLock::new(BiMap::<String, ObjectId>::new());
    let session_list = web::Data::new(session_list);
    println!("Server running on localhost 3000");

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(NormalizePath::default())
            .data(client.clone().database(DB))
            .app_data(session_list.clone())
            .configure(route::config)
        // .service(Files::new("/", "./public/root/").index_file("index.html"))
    })
    .bind(ADDR)?
    .run()
    .await
}
