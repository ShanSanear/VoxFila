use dioxus::prelude::*;
use dioxus_logger::tracing::{debug, error, info};

use shared::models::{NewSession, SessionDetails, SessionState};

#[cfg(feature = "db")]
use crate::database::get_db;

#[server]
pub async fn list_sessions() -> Result<Vec<SessionDetails>, ServerFnError> {
    info!("Listing all sessions from database");
    let db = get_db().await;
    let results = sqlx::query_as!(
        SessionDetails,
        "SELECT session_id, state, songs_per_singer, current_queue_entry_id, updated_at FROM sessions"
    )
    .fetch_all(db)
    .await?;
    info!("Returning {} sessions", results.len());
    Ok(results)
}

#[server]
pub async fn get_current_session() -> Result<Option<SessionDetails>, ServerFnError> {
    info!("Getting active session from database");
    let db = get_db().await;
    let result = sqlx::query_as!(
        SessionDetails,
        "SELECT session_id, state, songs_per_singer, current_queue_entry_id, updated_at FROM sessions WHERE state = 'current'"
    )
    .fetch_optional(db)
    .await?;
    info!("Returning active session: {:?}", result);
    Ok(result)
}

#[server]
pub async fn update_session_state(
    session_id: i32,
    update_session: SessionState,
) -> Result<(), ServerFnError> {
    info!("Updating session with ID: {}", session_id);
    let db = get_db().await;
    sqlx::query!(
        "UPDATE sessions SET state = $1 WHERE session_id = $2",
        update_session.to_string(),
        session_id
    )
    .execute(db)
    .await?;
    info!("Session updated successfully");
    Ok(())
}

#[server]
pub async fn update_session_songs_per_singer(
    session_id: i32,
    songs_per_singer: i32,
) -> Result<(), ServerFnError> {
    info!(
        "Updating session with ID: {} to have {} songs per singer",
        session_id, songs_per_singer
    );
    let db = get_db().await;
    sqlx::query!(
        "UPDATE sessions SET songs_per_singer = $1 WHERE session_id = $2",
        songs_per_singer,
        session_id
    )
    .execute(db)
    .await?;
    info!("Session updated successfully");
    Ok(())
}

#[server]
pub async fn update_session_current_queue_entry_id(
    session_id: i32,
    current_queue_entry_id: i32,
) -> Result<(), ServerFnError> {
    info!(
        "Updating session with ID: {} to have current queue entry ID: {}",
        session_id, current_queue_entry_id
    );
    let db = get_db().await;
    sqlx::query!(
        "UPDATE sessions SET current_queue_entry_id = $1 WHERE session_id = $2",
        current_queue_entry_id,
        session_id
    )
    .execute(db)
    .await?;
    info!("Session updated successfully");
    Ok(())
}

#[server]
pub async fn create_new_session(new_session: NewSession) -> Result<SessionDetails, ServerFnError> {
    info!("Creating new session");
    let db = get_db().await;
    let current_session = get_current_session().await?;
    match current_session {
        Some(session) => {
            update_session_state(session.session_id, SessionState::Finished).await?;
        }
        None => {
            debug!("No current session found");
        }
    }
    let result = sqlx::query_as!(
        SessionDetails,
        "INSERT INTO sessions (songs_per_singer, state) VALUES ($1, $2)
        RETURNING session_id, state, songs_per_singer, current_queue_entry_id, updated_at",
        new_session.songs_per_singer,
        SessionState::Current.to_string()
    )
    .fetch_one(db)
    .await;
    info!("New session created successfully");
    match result {
        Ok(session) => {
            info!("New session details: {:?}", session);
            Ok(session)
        }
        Err(e) => {
            error!("Error creating new session: {}", e);
            Err(ServerFnError::new("Failed to create new session"))
        }
    }
}
