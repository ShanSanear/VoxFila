#[cfg(feature = "db")]
pub mod database;

pub mod songs;
pub use songs::{list_songs, list_songs_dummy, search_songs};
