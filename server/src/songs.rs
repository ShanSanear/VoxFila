use dioxus::prelude::*;
use log::{debug, info};

use shared::models::{NewSong, SongDetails};

#[cfg(feature = "db")]
use crate::database::get_db;

#[server]
pub async fn list_songs() -> Result<Vec<SongDetails>, ServerFnError> {
    info!("Listing all songs from database");
    let db = get_db().await;
    let results = sqlx::query_as!(
        SongDetails,
        "SELECT song_id, artist, title, yturl, isingurl FROM songs"
    )
    .fetch_all(db)
    .await?;
    info!("Returning {} songs", results.len());
    Ok(results)
}

/// List songs where artist or title matches the query (case-insensitive, fuzzy)
#[server]
pub async fn search_songs(query: String) -> Result<Vec<SongDetails>, ServerFnError> {
    info!("Searching songs with query: {}", query);
    let db = get_db().await;
    let pattern = format!("%{}%", query);
    let results = sqlx::query_as!(
        SongDetails,
        "SELECT song_id, artist, title, yturl, isingurl FROM songs WHERE artist LIKE $1 OR title LIKE $1",
        pattern
    )
    .fetch_all(db)
    .await?;
    info!("Returning {} songs", results.len());
    Ok(results)
}

#[server]
pub async fn save_song(song: NewSong) -> Result<SongDetails, ServerFnError> {
    info!("Attempting to save song: {:?}", song);
    let db = get_db().await;
    let song = sqlx::query_as!(
        SongDetails,
        "INSERT INTO songs (artist, title, yturl, isingurl) VALUES ($1, $2, $3, $4)
    RETURNING song_id, artist, title, yturl, isingurl",
        song.artist,
        song.title,
        song.yturl,
        song.isingurl
    )
    .fetch_one(db)
    .await?;
    info!("Saved song with id: {}", song.song_id);
    Ok(song)
}
