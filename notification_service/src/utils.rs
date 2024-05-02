use crate::model::TokenClaims;

pub fn verify_jwt_token(
    token: &str
) -> Result<TokenClaims, jsonwebtoken::errors::Error> {
    let validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256);

    let decoded = jsonwebtoken::decode::<TokenClaims>(
        token,
        &jsonwebtoken::DecodingKey::from_rsa_pem(include_bytes!("public_key.pem"))?,
        &validation,
    );
    
    match decoded {
        Ok(c) => Ok(c.claims),
        Err(e) => return Err(e.into())
    }
}