pub mod database;
pub mod handler;
pub mod schema;
mod scope;

pub use scope::{scope, Ressource};

pub use schema::PlaylistRequest;
