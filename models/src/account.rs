use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Account {
    pub id: i32,
    pub username: String,
    pub gender: String,
    pub email: String,
    pub reg_date: chrono::NaiveDate,
    pub password: String,
    pub salt: String,
    pub introduction: String,
    pub avatar: String,
    pub role: i32,
}

#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize)]
pub struct Auth {
    id: String,
    account: i32,
    expire: i32,
}
