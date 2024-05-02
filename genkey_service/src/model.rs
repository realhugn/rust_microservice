use diesel::{Queryable, Selectable, Identifiable, Insertable};
use serde::{Serialize, Deserialize};

use crate::schema::access_key;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, PartialEq,Clone)]
#[diesel(table_name = access_key)]
pub struct AccessKey {
    pub id: i32,
    pub token: String,
    pub permission: Vec<Option<String>>
}

#[derive(Debug, Insertable, Deserialize, Serialize)]
#[diesel(table_name = access_key)]
pub struct NewAccessKey {
    pub token: String,
    pub permission: Vec<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SignPayload {
    pub uid: i32,
    pub allow: Vec<String>,
    pub exp: usize,
    pub permission: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub uid: i32,
    pub allow: Vec<String> ,
    pub iat: usize,
    pub exp: usize,
}