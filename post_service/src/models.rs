use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable};

use crate::schema::posts;
use crate::schema::users;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Post {
  pub id: i32,
  pub user_id: i32,
  pub title: String,
  pub description: String,
  pub created_at: chrono::NaiveDateTime,
  pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "posts"]
pub struct NewPost {
  pub user_id: i32,
  pub title: String,
  pub description: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PostPayload {
  pub title: String,
  pub description: String,
  pub recipients : Vec<i32>
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct TokenClaims {
  pub sub: i32,
  pub role: i32,
  pub iat: usize,
  pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize, Queryable,PartialEq, Clone)]
#[diesel(primary_key(user_id))]
pub struct User {
  pub user_id : i32,
  pub role : i32
}
