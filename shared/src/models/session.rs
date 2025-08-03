use serde::{Deserialize, Serialize};

use chrono::{DateTime, NaiveDateTime, Utc};

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug, sqlx::FromRow)]
pub struct NewSession {
    pub songs_per_singer: i32,
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug, sqlx::FromRow)]
pub struct SessionDetails {
    pub session_id: i32,
    pub state: String, // TODO, enum for state
    pub songs_per_singer: i32,
    pub current_queue_entry_id: Option<i32>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug, sqlx::FromRow)]
pub struct SessionUpdate {
    pub session_id: i32,
    pub state: Option<String>, // TODO, enum for state
    pub songs_per_singer: Option<i32>,
    pub current_queue_entry_id: Option<i32>,
}
