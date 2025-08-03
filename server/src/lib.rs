#[cfg(feature = "db")]
pub mod database;

pub mod songs;
pub use songs::{list_songs, search_songs};
