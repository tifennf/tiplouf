mod playlist;
mod track;
mod utils;

pub use playlist::{create_one, delete_one, get_all, get_one};
pub use track::{add_track, get_track, remove_track};

const DB: &str = "tiplouf";
const COLLECTION: &str = "playlist";
