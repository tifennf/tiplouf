pub mod database;
pub mod error;
pub mod middleware;
pub mod route;
pub mod schema;

use actix_cors::Cors;
use actix_web::{
    middleware::{Logger, NormalizePath},
    App, HttpServer,
};
use mongodb::Client;

const ADDR: &str = "localhost:3000";

pub async fn start(client: Client) -> std::io::Result<()> {
    println!("Server running on port 3000");

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(NormalizePath::default())
            .data(client.clone())
            .configure(route::config)
    })
    .bind(ADDR)?
    .run()
    .await
}
