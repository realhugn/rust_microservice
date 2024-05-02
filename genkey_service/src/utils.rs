use jsonwebtoken::{Header, encode};
use chrono::{DateTime, Utc};

use crate::model::TokenClaims;


pub fn generate_token(id: i32, allow: Vec<String>, exp: usize, _now : DateTime<Utc>) ->  Result<String, jsonwebtoken::errors::Error> {

    let iat = _now.timestamp() as usize;
    let exp = exp + _now.timestamp_millis() as usize;
    let claims = TokenClaims {
        uid: id,
        allow,
        iat,
        exp
    };
    let token = encode (
        &Header::new(jsonwebtoken::Algorithm::RS256),
        &claims,
        &jsonwebtoken::EncodingKey::from_rsa_pem(include_bytes!("private.pem"))?
    );
    token
}

pub fn verify_jwt_token(
    token: String
) -> Result<TokenClaims, jsonwebtoken::errors::Error> {

    let validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256);

    let decoded = jsonwebtoken::decode::<TokenClaims>(
        token.as_str().clone(),
        &jsonwebtoken::DecodingKey::from_rsa_pem(include_bytes!("public.pem"))?,
        &validation,
    );
    
    match decoded {
        Ok(c) => Ok(c.claims),
        Err(e) => return Err(e.into())
        }
}