# Tiplouf

That is a playlist manager api to be used with a frontend

To run the api, start mongod service, then ```cargo run --release``` ( install rust <https://www.rust-lang.org/tools/install> )

Api server port: localhost:3000 | change src/lib.rs ```const ADDR: &str = "localhost:3000";``` to your favorite one


## Information

Tiplouf is using Rust Actix-Web framework with a mongodb database, i made that api to exerce myself with Rust and api development at first, so do not expect fully tested api with tons of features.

### Api testing

First run the api + mongod, then you can run the "all_route" test with ```cargo test```


### Routes

First, you need to register then login, then add the session_id cookie to all your requests on /playlist

| Method | Path      | Body example | Description |
|--------|-----------|-----------|-----------|
| POST   | /auth/register | { "username": "xxxx", "password": "yyyyy" } | Register an user |
| POST   | /auth/login   | { "username": "xxxx", "password": "yyyyy" } | Login with user infos |



There is 7 playlist endpoints:
| Method | Path      | Body example | Description |
|--------|-----------|-----------|-----------|
| GET    | /playlist | None      | Get all your playlist |
| GET    | /playlist?tag="something"   |   None        | Get all your playlist with this tag |
| GET    | /playlist/{id}          |  None | Get one of your playlist using his ID        |
| POST    | /playlist          | { "tracklist": ["url1", "url2", "..."], "tag": "Classique" }      |  Add a new playlist related to you|
| POST   | /playlist/{id}/track       | ["url1", "url2", "..." ]         | Add tracks to your playlist |
| DELETE   | /playlist/{id}          | None          | Delete one of your playlist using his ID
| DELETE   | /playlist/{id}/track         | ["track_id1", "track_id2", "track_id3", "..."]        | Delete all track corresponding his id that are in your playlist
