#[cfg(feature = "db")]
use crate::database::get_db;

use dioxus::logger::tracing::{debug, info};
use dioxus::{
    logger::{self, tracing},
    prelude::*,
};

use shared::models::{NewSinger, SingerDetails, SingerUpdate};

#[server]
pub async fn list_singers() -> Result<Vec<SingerDetails>, ServerFnError> {
    info!("Listing all singers from database");
    let db = get_db().await;
    debug!("Using database: {:?}", db.connect_options());
    let result = sqlx::query_as!(SingerDetails, "SELECT singer_id, name FROM singers")
        .fetch_all(db)
        .await?;
    info!("Returning {} singers", result.len());
    Ok(result)
}

#[server]
pub async fn search_singers(query: String) -> Result<Vec<SingerDetails>, ServerFnError> {
    info!("Searching singers with query: {}", query);
    let db = get_db().await;
    let pattern = format!("%{}%", query);
    // COLLATE NOCASE for sqlite
    let singers = sqlx::query_as!(
        SingerDetails,
        "SELECT singer_id, name FROM singers WHERE name LIKE $1",
        pattern.as_str(),
    )
    .fetch_all(db)
    .await?;
    info!("Returning {} singers", singers.len());
    debug!("Singers found: {:?}", singers);
    Ok(singers)
}

#[server]
pub async fn get_singer(singer_name: String) -> Result<SingerDetails, ServerFnError> {
    info!("Searching singers with name: {}", singer_name);
    let db = get_db().await;

    let singers = sqlx::query_as!(
        SingerDetails,
        "SELECT singer_id, name FROM singers WHERE name = $1",
        singer_name.as_str(),
    )
    .fetch_one(db)
    .await?;

    debug!("Singers found: {:?}", singers);
    Ok(singers)
}

#[server]
pub async fn save_singer(singer: NewSinger) -> Result<SingerDetails, ServerFnError> {
    info!("Saving new singer: {:?}", singer);
    let db = get_db().await;
    let new_singer = sqlx::query_as!(
        SingerDetails,
        "INSERT INTO singers (name) VALUES ($1) RETURNING singer_id, name;",
        singer.name
    )
    .fetch_one(db)
    .await?;
    Ok(new_singer)
}

#[server]
pub async fn update_singer(singer: SingerUpdate) -> Result<SingerDetails, ServerFnError> {
    info!("Updating singer: {:?}", singer);
    let db = get_db().await;
    let updated_singer = sqlx::query_as!(
        SingerDetails,
        "UPDATE singers SET name = $1 WHERE singer_id = $2
    RETURNING singer_id, name;",
        singer.name,
        singer.singer_id as i32, // TODO why this needs casting to i32?
    )
    .fetch_one(db)
    .await?;

    Ok(updated_singer)
}
