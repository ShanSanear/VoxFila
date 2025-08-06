use dioxus::prelude::*;
use log::{debug, info};

use shared::models::{session, NewSong, QueueEntryDetails, SingerDetails, SongDetails};

use crate::singers::get_singer;

#[cfg(feature = "db")]
use crate::database::get_db;

#[server]
pub async fn create_queue_entry(
    session_id: i32,
    singer_name: String,
    song_id: i32,
    second_singer_name: Option<String>,
    notes: String,
) -> Result<i32, ServerFnError> {
    let db = get_db().await;
    let singer = get_singer(singer_name).await?;

    let second_singer = if let Some(second_singer_name) = second_singer_name {
        Some(get_singer(second_singer_name).await?)
    } else {
        None
    };

    let result = match second_singer {
        Some(second_singer) => {
            sqlx::query_scalar!(
                "INSERT INTO queue_entries (session_id, singer_id, song_id, second_singer_id, notes)
                 VALUES ($1, $2, $3, $4, $5)
                 RETURNING queue_entry_id",
                session_id,
                singer.singer_id,
                song_id,
                second_singer.singer_id,
                notes
            )
        }
        None => {
            sqlx::query_scalar!(
                "INSERT INTO queue_entries (session_id, singer_id, song_id, notes)
                 VALUES ($1, $2, $3, $4)
                 RETURNING queue_entry_id",
                session_id,
                singer.singer_id,
                song_id,
                notes
            )
        }
    };
    let out = result.fetch_one(db).await?;

    info!("Returning {} queue_entry", out);
    Ok(out)
}

#[server]
pub async fn get_queue_entry(
    session_id: i32,
    queue_entry_id: i32,
) -> Result<QueueEntryDetails, ServerFnError> {
    let db = get_db().await;
    let result: QueueEntryDetails = sqlx::query_as(
        r#"SELECT qe.queue_entry_id,
qe.session_id,
s.singer_id,
s.name,
ss.singer_id as secondary_singer_id,
ss.name as secondary_singer_name,
son.song_id,
son.artist,
son.title,
son.yturl,
son.isingurl,
qe.status,
qe.queue_position,
qe.original_position,
qe.notes,
qe.moved_at
FROM queue_entries qe
JOIN singers s on qe.singer_id = s.singer_id
JOIN singers ss on qe.second_singer_id = ss.singer_id
JOIN songs son on qe.song_id = son.song_id
WHERE qe.session_id=$1 AND qe.queue_entry_id=$2;"#,
    )
    .bind(session_id)
    .bind(queue_entry_id)
    .fetch_one(db)
    .await?;

    debug!("Returning song details for queue entry {}", queue_entry_id);
    Ok(result)
}
