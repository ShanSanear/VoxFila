#[cfg(feature = "db")]
pub mod database;

pub mod songs;
pub use songs::{get_song, list_songs, list_songs_dummy, search_songs};
