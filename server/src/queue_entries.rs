use dioxus::prelude::*;
use dioxus_logger::tracing::{debug, info};

use crate::sessions::get_current_session;
use crate::singers::get_or_create_singer;
use shared::models::QueueEntryDetails;

#[cfg(feature = "db")]
use crate::database::get_db;

#[server]
pub async fn create_queue_entry(
    singer_name: String,
    song_id: i32,
    second_singer_name: Option<String>,
    notes: String,
) -> Result<i32, ServerFnError> {
    let db = get_db().await;
    let session = get_current_session().await?;
    let mut session_id = -1;
    match session {
        Some(session) => {
            session_id = session.session_id;
        }
        None => {
            return Err(ServerFnError::new("No active session found"));
        }
    }
    let singer = get_or_create_singer(singer_name).await?;

    let second_singer = if let Some(second_singer_name) = second_singer_name {
        Some(get_or_create_singer(second_singer_name).await?)
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
pub async fn get_queue_entry(queue_entry_id: i32) -> Result<QueueEntryDetails, ServerFnError> {
    let db = get_db().await;
    let result: QueueEntryDetails = sqlx::query_as(
        r#"SELECT qe.queue_entry_id,
qe.session_id,
s.singer_id,
s.name,
ss.singer_id as second_singer_id,
ss.name as second_singer_name,
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
WHERE qe.queue_entry_id=$1;"#,
    )
    .bind(queue_entry_id)
    .fetch_one(db)
    .await?;

    debug!("Returning song details for queue entry {}", queue_entry_id);
    Ok(result)
}

#[server]
pub async fn list_queue_entries() -> Result<Vec<QueueEntryDetails>, ServerFnError> {
    let session = get_current_session().await?;
    let mut session_id = -1;
    match session {
        Some(session) => {
            session_id = session.session_id;
        }
        None => {
            return Err(ServerFnError::new("No active session found"));
        }
    }
    let db = get_db().await;
    let result: Vec<QueueEntryDetails> = sqlx::query_as(
        r#"SELECT qe.queue_entry_id,
qe.session_id,
s.singer_id,
s.name,
ss.singer_id as second_singer_id,
ss.name as second_singer_name,
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
LEFT JOIN singers ss on qe.second_singer_id = ss.singer_id
JOIN songs son on qe.song_id = son.song_id
WHERE qe.session_id=$1;"#,
    )
    .bind(session_id)
    .fetch_all(db)
    .await?;

    debug!(
        "Returning {} queue entries for session {}",
        result.len(),
        session_id
    );
    Ok(result)
}

#[server]
pub async fn list_pending_queue_entries() -> Result<Vec<QueueEntryDetails>, ServerFnError> {
    let session = get_current_session().await?;
    let mut session_id = -1;
    match session {
        Some(session) => {
            session_id = session.session_id;
        }
        None => {
            return Err(ServerFnError::new("No active session found"));
        }
    }
    let db = get_db().await;
    let result: Vec<QueueEntryDetails> = sqlx::query_as(
        r#"SELECT qe.queue_entry_id,
qe.session_id,
s.singer_id,
s.name,
ss.singer_id as second_singer_id,
ss.name as second_singer_name,
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
LEFT JOIN singers ss on qe.second_singer_id = ss.singer_id
JOIN songs son on qe.song_id = son.song_id
WHERE qe.session_id=$1 AND qe.status='pending'
ORDER BY qe.queue_position;"#,
    )
    .bind(session_id)
    .fetch_all(db)
    .await?;

    debug!(
        "Returning {} queue entries for session {}",
        result.len(),
        session_id
    );
    Ok(result)
}

#[server]
pub async fn remove_queue_entry(queue_entry_id: i32) -> Result<(), ServerFnError> {
    let db = get_db().await;
    sqlx::query("DELETE FROM queue_entries WHERE queue_entry_id=$1")
        .bind(queue_entry_id)
        .execute(db)
        .await?;
    Ok(())
}

#[server]
pub async fn complete_queue_entry<'a>(queue_entry_id: i32) -> Result<(), ServerFnError> {
    let db = get_db().await;
    sqlx::query("UPDATE queue_entries SET status='completed' WHERE queue_entry_id=$1")
        .bind(queue_entry_id)
        .execute(db)
        .await?;
    Ok(())
}

#[server]
pub async fn move_queue_entry(queue_entry_id: i32, new_position: i32) -> Result<(), ServerFnError> {
    let db = get_db().await;
    sqlx::query("UPDATE queue_entries SET queue_position=$1 WHERE queue_entry_id=$2")
        .bind(new_position)
        .bind(queue_entry_id)
        .execute(db)
        .await?;
    Ok(())
}
