use dioxus::prelude::*;
use dioxus_logger::tracing::{debug, info};

use shared::models::{NewSong, SongDetails};

#[cfg(feature = "db")]
use crate::database::get_db;
#[cfg(feature = "db")]
use sqlx::QueryBuilder;

#[server]
pub async fn list_songs() -> Result<Vec<SongDetails>, ServerFnError> {
    info!("Listing all songs from database");
    let db = get_db().await;
    let results = sqlx::query_as!(
        SongDetails,
        "SELECT song_id, artist, title, yturl, isingurl FROM songs ORDER BY (artist, title)"
    )
    .fetch_all(db)
    .await?;
    info!("Returning {} songs", results.len());
    Ok(results)
}

#[server]
pub async fn get_song(id: i32) -> Result<SongDetails, ServerFnError> {
    info!("Fetching song with id: {}", id);
    let db = get_db().await;
    let song = sqlx::query_as!(
        SongDetails,
        "SELECT song_id, artist, title, yturl, isingurl FROM songs WHERE song_id = $1",
        id
    )
    .fetch_one(db)
    .await?;
    info!("Found song: {:?}", song);
    Ok(song)
}

/// List songs where artist or title matches the query (case-insensitive, fuzzy)
#[server]
pub async fn search_songs(query: String, limit: i64) -> Result<Vec<SongDetails>, ServerFnError> {
    info!("Searching songs with query: {}", query);
    let db = get_db().await;
    let pattern = format!("%{}%", query);

    let result = if query.is_empty() {
        sqlx::query_as!(
            SongDetails,
            "SELECT song_id, artist, title, yturl, isingurl FROM songs ORDER BY (artist, title) LIMIT $1",
            limit
        )
        .fetch_all(db)
        .await?
    } else {
        sqlx::query_as!(
            SongDetails,
            "SELECT song_id, artist, title, yturl, isingurl FROM songs WHERE $1 <% (artist || ' ' || title) ORDER BY (artist, title) LIMIT $2",
            pattern,
            limit
        )
        .fetch_all(db)
        .await?
    };

    info!("Returning {} songs", result.len());
    Ok(result)
}

#[server]
pub async fn search_song_by_artist(query: String) -> Result<Vec<SongDetails>, ServerFnError> {
    info!("Searching songs by artist with query: {}", query);
    let db = get_db().await;

    let results = if query.is_empty() {
        sqlx::query_as!(
            SongDetails,
            "SELECT song_id, artist, title, yturl, isingurl FROM songs ORDER BY (artist, title)"
        )
        .fetch_all(db)
        .await?
    } else {
        sqlx::query_as!(
            SongDetails,
            "SELECT song_id, artist, title, yturl, isingurl FROM songs WHERE artist LIKE $1 ORDER BY (artist, title)",
            query
        )
        .fetch_all(db)
        .await?
    };
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

#[server]
pub async fn import_songs(songs: Vec<NewSong>) -> Result<(), ServerFnError> {
    info!("Importing songs from external source");
    let db = get_db().await;
    let mut query_builder = QueryBuilder::new(
        "
    INSERT INTO songs (artist, title, yturl, isingurl) ",
    );
    query_builder.push_values(songs, |mut b, song| {
        b.push_bind(song.artist)
            .push_bind(song.title)
            .push_bind(song.yturl)
            .push_bind(song.isingurl);
    });
    query_builder.push(" ON CONFLICT (artist, title) DO NOTHING");
    let query = query_builder.build();
    query.execute(db).await?;

    Ok(())
}
