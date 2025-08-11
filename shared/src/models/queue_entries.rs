use serde::{Deserialize, Serialize};

use chrono::NaiveDateTime;

use crate::models::{SecondSingerDetails, SingerDetails, SongDetails};

use sqlx;

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub enum QueueEntryStatus {
    Pending,
    Current,
    Completed,
    Skipped,
    Priority,
}
impl From<String> for QueueEntryStatus {
    fn from(s: String) -> Self {
        match s.as_str() {
            "pending" => QueueEntryStatus::Pending,
            "current" => QueueEntryStatus::Current,
            "completed" => QueueEntryStatus::Completed,
            "skipped" => QueueEntryStatus::Skipped,
            "priority" => QueueEntryStatus::Priority,
            _ => QueueEntryStatus::Pending,
        }
    }
}

impl std::fmt::Display for QueueEntryStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            QueueEntryStatus::Pending => write!(f, "pending"),
            QueueEntryStatus::Current => write!(f, "current"),
            QueueEntryStatus::Completed => write!(f, "completed"),
            QueueEntryStatus::Skipped => write!(f, "skipped"),
            QueueEntryStatus::Priority => write!(f, "priority"),
        }
    }
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug, sqlx::FromRow)]
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

impl QueueEntryDetails {
    pub fn is_current(&self) -> bool {
        self.status == QueueEntryStatus::Current.to_string()
    }
    pub fn actual_status(&self) -> QueueEntryStatus {
        QueueEntryStatus::from(self.status.clone())
    }
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub struct QueueEntryUpdate {
    pub queue_entry_id: i32,
    pub session_id: Option<i32>,
    pub singer_id: Option<i32>,
    pub song_id: Option<i32>,
}
