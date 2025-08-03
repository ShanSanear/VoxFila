use serde::{Deserialize, Serialize};

use chrono::{DateTime, NaiveDateTime, Utc};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NewSong {
    pub artist: String,
    pub title: String,
    pub yturl: Option<String>,
    pub isingurl: Option<String>,
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug, sqlx::FromRow)]
pub struct SongDetails {
    pub song_id: i32,
    pub artist: String,
    pub title: String,
    pub yturl: Option<String>,
    pub isingurl: Option<String>,
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub struct SongUpdate {
    pub song_id: i32,
    pub artist: Option<String>,
    pub title: Option<String>,
    pub yturl: Option<String>,
    pub isingurl: Option<String>,
}
