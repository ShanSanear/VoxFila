use serde::{Deserialize, Serialize};

use chrono::{DateTime, NaiveDateTime, Utc};

use crate::models::{SecondSingerDetails, SingerDetails, SongDetails};

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct NewQueueEntry {
    pub session_id: i32,
    pub singer_name: String,
    pub second_singer_name: Option<String>,
    pub song_id: i32,
    pub notes: Option<String>,
}
#[derive(Clone, PartialEq, Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct QueueEntryDetails {
    pub queue_entry_id: i32,
    pub session_id: i32,
    #[sqlx(flatten)]
    pub singer: SingerDetails,
    #[sqlx(flatten)]
    pub second_singer: SecondSingerDetails,
    #[sqlx(flatten)]
    pub song: SongDetails,
    pub status: String,
    pub queue_position: Option<i32>, // 0 = currently singing, 1-n = in queue
    pub original_position: Option<i32>,
    pub notes: Option<String>,
    pub moved_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub struct QueueEntryUpdate {
    pub queue_entry_id: i32,
    pub session_id: Option<i32>,
    pub singer_id: Option<i32>,
    pub song_id: Option<i32>,
}
