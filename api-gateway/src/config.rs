use regex::Regex;
use serde::{Deserialize, Serialize};
use serde::de::{self, Deserializer};
use serde::ser::Serializer;
use std::{fs::File, io::Read};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
use toml;

fn deserialize_regex<'de, D>(deserializer: D) -> Result<Regex, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Regex::new(&s).map_err(de::Error::custom)
}

fn serialize_regex<S>(regex: &Regex, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(regex.as_str())
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServiceConfig {
    #[serde(deserialize_with = "deserialize_regex", serialize_with = "serialize_regex")]
    pub path: Regex,
    pub target_service: String,
    pub target_port: String,
    pub authentication_required: Option<bool>,
    pub ratelimit_required: Option<bool>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GatewayConfig {
    pub authorization_api_url: String,
    pub services: Vec<ServiceConfig>,
}

pub fn load_config(path: &str) -> GatewayConfig {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    toml::from_str(&contents).unwrap()
}

pub fn establish_connection_pool() -> DbPool {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database connection pool");
    pool
}

// #[cfg(test)]
// pub fn establish_connection_pool_test() -> DbPool {
//     dotenv().ok();

//     let database_url = std::env::var("DATABASE_URL_TEST")
//         .expect("DATABASE_URL_TEST must be set");
//     let manager = ConnectionManager::<PgConnection>::new(database_url);
//     let pool = r2d2::Pool::builder()
//         .build(manager)
//         .expect("Failed to create database connection pool");
//     pool
// }