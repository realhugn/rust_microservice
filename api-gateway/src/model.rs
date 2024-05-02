use diesel::{Queryable, Insertable, AsChangeset, Identifiable, Selectable};
use serde::{Deserialize, Serialize};

use crate::schema::access_request;

#[derive(Debug, Serialize, Deserialize,Queryable, Selectable, Identifiable, PartialEq,Clone)]
#[diesel(primary_key(id))]
#[diesel(table_name = access_request)]
pub struct AccessRequest {
    pub id : i32,
    pub user_id: i32,
    pub count: i32, 
    pub time: chrono::NaiveDateTime
}

#[derive(Debug, Insertable, Deserialize, Serialize)]
#[diesel(table_name = access_request)]
pub struct NewLog {
    pub user_id : i32, 
    pub count: i32
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, Clone)]
#[diesel(table_name = access_request)]
pub struct UpdateLog {
    pub count : i32,
    pub time: chrono::NaiveDateTime
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: i32,
    pub role: i32,
    pub iat: usize,
    pub exp: usize,
}

