pub mod queue_entries;
pub mod session;
pub mod singers;
pub mod songs;
pub use queue_entries::{NewQueueEntry, QueueEntryDetails, QueueEntryUpdate};
pub use session::{NewSession, SessionDetails, SessionUpdate};
pub use singers::{NewSinger, SecondSingerDetails, SingerDetails, SingerUpdate};
pub use songs::{NewSong, SongDetails, SongUpdate};
