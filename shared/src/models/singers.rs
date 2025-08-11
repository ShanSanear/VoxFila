use serde::{Deserialize, Serialize};


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
    /// This struct is a workaround for dealing with how sqlx handles `flatten` type of conversion
    /// from sql row into a struct
    pub second_singer_id: Option<i32>,
    pub second_singer_name: Option<String>,
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug, sqlx::FromRow)]
pub struct SingerUpdate {
    pub singer_id: i32,
    pub name: String,
}
