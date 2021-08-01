pub mod playlist;
pub mod route;
pub mod shared;
pub mod track;
pub mod user;

use std::sync::RwLock;

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{
    middleware::{Logger, NormalizePath},
    web, App, HttpServer,
};
use bimap::BiMap;
use dashmap::{DashMap, DashSet};
use mongodb::{bson::oid::ObjectId, Client};
use shared::ApiError;
// use tokio::sync::RwLock;

const ADDR: &str = "localhost:3000";
const DB: &str = "tiplouf";

pub async fn start(client: Client) -> std::io::Result<()> {
    let session_list = RwLock::new(BiMap::<String, ObjectId>::new());
    let session_list = web::Data::new(session_list);
    println!("Server running on port 3000");

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(NormalizePath::default())
            .data(client.clone().database(DB))
            .app_data(session_list.clone())
            .configure(route::config)
            .service(Files::new("/", "./public/root/").index_file("index.html"))
    })
    .bind(ADDR)?
    .run()
    .await
}

// pub struct SessionList {
//     s_id_list: DashMap<String, ObjectId>,
//     u_id_list: DashMap<ObjectId, String>,
// }

// impl SessionList {
//     pub fn new() -> SessionList {
//         let s_id_list = DashSet::new();
//         let u_id_list = DashSet::new();

//         SessionList {
//             s_id_list,
//             u_id_list,
//         }
//     }

//     pub fn get(session_id: String) -> ObjectId {




//     }

//     pub fn insert(&self, session_id: String, user_id: ObjectId) -> Result<(), ApiError>{
//         if self.u_id_list.insert(user_id, user) && self.s_id_list.insert(session_id) {
//             Ok(())
//         } else {
//             Err(ApiError::ValidationError("Already logged in".into()))
//         }
//     }
// }



// i prefer not using actix test module, but i let that comment for notes
// #[cfg(test)]
// mod test {
//     use super::*;
//     use actix_web::{dev::ResponseBody, http::Method, test};

//     #[actix_rt::test]
//     async fn api_test() {
//         let client = Client::with_uri_str("mongodb://127.0.0.1:27017/")
//             .await
//             .expect("failed to connect DB");

//         let cors = Cors::permissive();

//         let mut app = test::init_service(
//             App::new()
//                 .wrap(cors)
//                 .wrap(Logger::default())
//                 .wrap(NormalizePath::default())
//                 .data(client.clone().database(DB))
//                 .configure(route::config)
//                 .service(Files::new("/", "./public/root/").index_file("index.html")),
//         )
//         .await;

//         let get_playlist = test::TestRequest::get().uri("/playlist")
//             .to_request();
//         let get_playlist = test::call_service(&mut app, get_playlist).await;
//         assert!(get_playlist.status().is_success());

//         let post_playlist = test::TestRequest::post().uri("/playlist")
//             .to_request();
//         let post_playlist = test::call_service(&mut app, post_playlist).await;
//         assert!(post_playlist.status().is_success());

//     }

// }
