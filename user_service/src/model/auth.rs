use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    pub sub: i32,
    pub role: i32,
    pub iat: usize,
    pub exp: usize,
}