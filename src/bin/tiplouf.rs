use mongodb::Client;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let client = Client::with_uri_str("mongodb://127.0.0.1:27017/")
        .await
        .expect("failed to connect DB");

    tiplouf::start(client).await
}
