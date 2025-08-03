use dioxus::prelude::*;
use log::info;

use sqlx::PgPool;
use sqlx::SqlitePool;
use std::env;

use tokio::sync::OnceCell;

static DB: OnceCell<PgPool> = OnceCell::const_new();

async fn init_db() -> PgPool {
    let db_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:4222/postgres".to_string());
    PgPool::connect(&db_url)
        .await
        .expect("Failed to connect to database")
}

pub async fn get_db() -> &'static PgPool {
    DB.get_or_init(init_db).await
}
