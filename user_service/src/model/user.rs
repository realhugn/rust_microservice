use chrono::Utc;
use diesel::{Queryable, Insertable, AsChangeset, Identifiable, Selectable};
use random_string::generate;
use serde::{Deserialize, Serialize};

use crate::{schema::users, CHARSET, utils::hash_sha256};

#[derive(Debug, Serialize, Deserialize,Queryable, Selectable, Identifiable, PartialEq,Clone)]
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

#[derive(Debug, Serialize, Deserialize, AsChangeset, Clone)]
#[diesel(table_name = users)]
pub struct UpdateUser{
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub phone: String,
    pub status: i32,
    pub updated_at: chrono::NaiveDateTime,
    pub role: i32
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateUserPayload {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub phone: String,
    pub status: i32,
    pub role: i32
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChangePasswordPayload {
    pub password: String,
    pub old_password :String
}

impl UpdateUser {
    pub fn new(payload: UpdateUserPayload) -> Self{
        let now = Utc::now().naive_utc();
        Self { 
            firstname: (payload.firstname), 
            lastname: (payload.lastname), 
            email: (payload.email), 
            phone: (payload.phone), 
            status: (payload.status), 
            updated_at: (now) ,
            role: (payload.role)
        }
    }
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


