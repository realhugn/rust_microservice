use diesel::{Queryable, Insertable};
use serde::{Deserialize, Serialize};

use crate::schema::notifications;

#[derive(Debug, Serialize, Deserialize, Queryable)]
#[diesel(table_name = notifications)]
pub struct Notification {
    pub id : i32,
    pub user_id: i32,
    pub description: String,
    pub title: String, 
    #[serde(rename = "type")]
    pub type_: i32,
    pub entity_id: String,
    pub created_at: chrono::NaiveDateTime
}  


#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = notifications)]
pub struct NewNotification {
    pub user_id : i32, 
    pub description: String,
    pub title: String, 
    pub type_: i32,
    pub entity_id: String,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct TokenClaims {
    pub sub: i32,
    pub role: i32,
    pub iat: usize,
    pub exp: usize,
}
