use std::str::FromStr;

use serde::{Deserialize, Serialize};

use chrono::{DateTime, NaiveDateTime, Utc};

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub enum SessionState {
    New,
    Current,
    Paused,
    Finished,
}

impl From<String> for SessionState {
    fn from(s: String) -> Self {
        match s.as_str() {
            "new" => SessionState::New,
            "current" => SessionState::Current,
            "paused" => SessionState::Paused,
            "finished" => SessionState::Finished,
            _ => panic!("Unknown session state: {}", s),
        }
    }
}

impl std::fmt::Display for SessionState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SessionState::New => write!(f, "new"),
            SessionState::Current => write!(f, "current"),
            SessionState::Paused => write!(f, "paused"),
            SessionState::Finished => write!(f, "finished"),
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug, sqlx::FromRow)]
pub struct NewSession {
    pub songs_per_singer: i32,
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug, sqlx::FromRow)]
pub struct SessionDetails {
    pub session_id: i32,
    pub state: SessionState,
    pub songs_per_singer: i32,
    pub current_queue_entry_id: Option<i32>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug, sqlx::FromRow)]
pub struct SessionUpdate {
    pub session_id: i32,
    pub state: Option<SessionState>,
    pub songs_per_singer: Option<i32>,
    pub current_queue_entry_id: Option<i32>,
}
