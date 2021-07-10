use derive_more::{Display, Error};
use std::io;

pub enum Field {
    Playlist(Playlist),
}

#[derive(Debug, Display)]
pub enum Playlist {
    #[display(fmt = "Playlist not found")]
    NotFound,
    #[display(fmt = "Invalid playlist id")]
    InvalidId,
    #[display(fmt = "Track not found")]
    TrackNotFound,
    #[display(fmt = "Invalid track id")]
    TrackInvalidId,
}
