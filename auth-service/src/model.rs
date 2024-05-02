use chrono::Utc;
use diesel::{Selectable, Identifiable, Queryable, Insertable};
use random_string::generate;
use serde::{Serialize, Deserialize};

use crate::{schema::users, utils::hash_sha256, CHARSET, schema::sessions};

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, PartialEq,Clone)]
#[diesel(primary_key(user_id))]
pub struct User {
    pub user_id : i32,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    #[serde(skip_serializing)]
    pub recent_password: Option<String>,
    pub firstname:String,
    pub lastname: String,
    pub email: String,
    pub phone: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub status: i32,
    #[serde(skip_serializing)]
    pub salt: String,
    pub role: i32
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, PartialEq,Clone)]
#[diesel(primary_key(session_id))]
pub struct Session {
    pub session_id: i32,
    pub user_id: i32,
    pub role: i32,
    pub expired_date: chrono::NaiveDateTime,
    pub token: String
}

#[derive(Debug, Serialize, Deserialize,Insertable, Clone)]
#[diesel(table_name = sessions)]
pub struct NewSession {
    pub user_id: i32,
    pub role: i32,
    pub expired_date: chrono::NaiveDateTime,
    pub token: String
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: i32,
    pub role: i32,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginPayload {
    pub username : String, 
    pub password : String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserPayload {
    pub username: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub phone: String,
    pub status: i32,
    pub role: i32
}

#[derive(Debug, Serialize, Deserialize,Insertable, Clone)]
#[diesel(table_name = users)]
pub struct NewUser{
    pub username: String,
    pub password: String, 
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub phone: String,
    pub status: i32,
    pub salt: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub role: i32
}

impl NewUser {
    pub fn new(payload : UserPayload) -> Self {
        let salt1 = generate(6, CHARSET);
        let hash_password = hash_sha256(payload.password, salt1.clone());
        let now = Utc::now().naive_utc();

        Self { 
            username: (payload.username), 
            password: (hash_password), 
            firstname: (payload.firstname), 
            lastname: (payload.lastname), 
            email: (payload.email), 
            phone: (payload.phone), 
            status: (payload.status), 
            salt: (salt1), 
            created_at: (now), 
            updated_at: (now),
            role: (payload.role)
        }
    }
}