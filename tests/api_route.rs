// YOU need to start mongod, then `cargo run` to start the server, then you can `cargo test`

use reqwest::Response;
use reqwest::StatusCode;

use reqwest::header;
use serde_json::Value;
use std::collections::HashSet;

use fake::{Dummy, Fake, Faker};
use serde::{Deserialize, Serialize};
use serde_json::json;

const IP: &str = "http://127.0.0.1:3000";

#[derive(Debug, Serialize, Deserialize, PartialEq, Dummy)]
struct User {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct ApiResponse<T> {
    status: String,
    data: T,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Playlist {
    tracklist: Vec<Track>,
    tag: Option<String>,
    p_id: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Track {
    url: String,
    p_id: String,
    t_id: String,
}

fn tracklist_example() -> HashSet<String> {
    let mut tracklist = HashSet::new();
    tracklist.insert("track1".to_string());
    tracklist.insert("track2".to_string());
    tracklist.insert("track3".to_string());

    tracklist
}

fn playlist_example() -> Value {
    let tracklist = tracklist_example();

    json!({
        "tracklist": tracklist,
        "tag": "classique"
    })
}

async fn post_one_playlist(client: &reqwest::Client, url: &str, session_id: &str) -> Response {
    let body = playlist_example();

    client
        .post(url)
        .header(header::COOKIE, session_id)
        .json(&body)
        .send()
        .await
        .unwrap()
}

async fn register_then_login(client: &reqwest::Client) -> String {
    let user: User = Faker.fake();

    let url = format!("{}/auth/register", IP);

    let res = client.post(url).json(&user).send().await.unwrap();

    assert!(res.status().is_success());

    let url = format!("{}/auth/login", IP);

    let res = client.post(url).json(&user).send().await.unwrap();

    let session_id = res
        .cookies()
        .reduce(|a, b| if a.name() == "session_id" { a } else { b })
        .unwrap();

    let session_id = format!("{}={}", session_id.name(), session_id.value());
    println!("{}", session_id);

    session_id
}

////////////////////////

#[tokio::test]
async fn all_route() {
    let client = reqwest::Client::new();
    let session_id = register_then_login(&client).await;

    get_playlist(&client, &session_id).await;
    post_playlist(&client, &session_id).await;
    get_one_playlist(&client, &session_id).await;
    delete_playlist(&client, &session_id).await;
    post_track(&client, &session_id).await;
    delete_track(&client, &session_id).await;
}

async fn get_playlist(client: &reqwest::Client, session_id: &str) {
    let url = format!("{}/playlist", IP);
    let res = client
        .get(url)
        .header(header::COOKIE, session_id)
        .send()
        .await
        .unwrap();

    let status = res.status();

    let playlist = res.json::<ApiResponse<Vec<Playlist>>>().await.unwrap();

    assert!(status.is_success());
    assert_eq!(playlist.status, "success");
}

async fn post_playlist(client: &reqwest::Client, session_id: &str) {
    let url = format!("{}/playlist", IP);
    let res = post_one_playlist(client, &url, session_id).await;

    let status = res.status();
    let playlist = res.json::<ApiResponse<Playlist>>().await.unwrap();

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(playlist.status, "success");
}

async fn get_one_playlist(client: &reqwest::Client, session_id: &str) {
    let url = format!("{}/playlist", IP);
    let playlist1 = post_one_playlist(client, &url, session_id)
        .await
        .json::<ApiResponse<Playlist>>()
        .await
        .unwrap();

    let id = &playlist1.data.p_id;
    let url = format!("{}/{}", url, id);

    let res = client
        .get(url)
        .header(header::COOKIE, session_id)
        .send()
        .await
        .unwrap();

    let status = res.status();

    let playlist2 = res.json::<ApiResponse<Playlist>>().await.unwrap();

    assert!(status.is_success());
    assert_eq!(playlist1.status, "success");
    assert_eq!(playlist1, playlist2);
}

async fn delete_playlist(client: &reqwest::Client, session_id: &str) {
    let url = format!("{}/playlist", IP);
    let playlist1 = post_one_playlist(client, &url, session_id)
        .await
        .json::<ApiResponse<Playlist>>()
        .await
        .unwrap();

    let id = &playlist1.data.p_id;
    let url = format!("{}/{}", url, id);

    let res = client
        .delete(url)
        .header(header::COOKIE, session_id)
        .send()
        .await
        .unwrap();

    let status = res.status();
    let playlist2 = res.json::<ApiResponse<Playlist>>().await.unwrap();

    assert_eq!(status, StatusCode::ACCEPTED);
    assert_eq!(playlist2.status, "success");
    assert_eq!(playlist1, playlist2);
}

async fn post_track(client: &reqwest::Client, session_id: &str) {
    let url = format!("{}/playlist", IP);
    let playlist1 = post_one_playlist(client, &url, session_id)
        .await
        .json::<ApiResponse<Playlist>>()
        .await
        .unwrap();

    let id = &playlist1.data.p_id;
    let url = format!("{}/{}/track", url, id);

    let track = tracklist_example();

    let res = client
        .post(url)
        .header(header::COOKIE, session_id)
        .json(&track)
        .send()
        .await
        .unwrap();

    let status = res.status();
    let playlist2 = res.json::<ApiResponse<Playlist>>().await.unwrap();

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(playlist2.status, "success");
    assert_ne!(playlist1, playlist2);
}

async fn delete_track(client: &reqwest::Client, session_id: &str) {
    let url = format!("{}/playlist", IP);
    let playlist1 = post_one_playlist(client, &url, session_id)
        .await
        .json::<ApiResponse<Playlist>>()
        .await
        .unwrap();

    let id = &playlist1.data.p_id;
    let url = format!("{}/{}/track", url, id);

    let id_list = playlist1
        .data
        .tracklist
        .iter()
        .map(|t| t.p_id.clone())
        .collect::<HashSet<String>>();

    let res = client
        .delete(url)
        .header(header::COOKIE, session_id)
        .json(&id_list)
        .send()
        .await
        .unwrap();

    let status = res.status();
    let playlist2 = res.json::<ApiResponse<Playlist>>().await.unwrap();

    assert_eq!(status, StatusCode::ACCEPTED);
    assert_eq!(playlist2.status, "success");
    assert_ne!(playlist1, playlist2);
}
