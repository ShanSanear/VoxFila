#[cfg(feature = "db")]
pub mod database;

pub mod songs;
pub use songs::{get_song, list_songs, list_songs_dummy, search_songs};

pub mod queue_entries;
pub use queue_entries::{
    complete_queue_entry, create_queue_entry, get_queue_entry, list_pending_queue_entries,
    list_queue_entries, remove_queue_entry,
};

pub mod singers;
pub use singers::{get_or_create_singer, get_singer, list_singers};

pub mod sessions;
pub use sessions::{get_active_session, list_sessions};
