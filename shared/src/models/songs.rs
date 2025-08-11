use serde::{Deserialize, Serialize};


use crate::utils::validation::input_has_valid_characters;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NewSong {
    pub artist: String,
    pub title: String,
    pub yturl: Option<String>,
    pub isingurl: Option<String>,
}

impl NewSong {
    pub fn validate(&self) -> bool {
        input_has_valid_characters(&self.artist)
            && input_has_valid_characters(&self.title)
            && self
                .yturl
                .as_ref()
                .is_none_or(|url| input_has_valid_characters(url))
            && self
                .isingurl
                .as_ref()
                .is_none_or(|url| input_has_valid_characters(url))
    }
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
