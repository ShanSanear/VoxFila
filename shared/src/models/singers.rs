use serde::{Deserialize, Serialize};

use chrono::{DateTime, NaiveDateTime, Utc};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NewSinger {
    pub name: String,
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug, sqlx::FromRow)]
pub struct SingerDetails {
    pub singer_id: i32,
    pub name: String,
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug, sqlx::FromRow)]
pub struct SecondSingerDetails {
    pub second_singer_id: i32,
    pub second_singer_name: String,
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug, sqlx::FromRow)]
pub struct SingerUpdate {
    pub singer_id: i32,
    pub name: String,
}
