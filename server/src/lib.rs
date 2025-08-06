#[cfg(feature = "db")]
pub mod database;

pub mod songs;
pub use songs::{get_song, list_songs, list_songs_dummy, search_songs};

pub mod queue_entries;
pub use queue_entries::{create_queue_entry, get_queue_entry};

pub mod singers;
pub use singers::{get_singer, list_singers};
