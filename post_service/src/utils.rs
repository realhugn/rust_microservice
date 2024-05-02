use diesel::{PgConnection, ExpressionMethods};
use diesel::prelude::*;

use crate::models::{TokenClaims, User};

type DbError = Box<dyn std::error::Error + Send + Sync>;

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

pub fn validate_user_role(user_ids: Vec<i32>, conn: &mut PgConnection, matches : Vec<i32> ) -> Result<bool, DbError> {
    use crate::schema::users::dsl::*;
    let list_users: Vec<User> = users
        .select((user_id, role))
        .filter(user_id.eq_any(user_ids.clone()))
        .load::<User>(conn)?;
    //check if user not exist in db 
    if list_users.len() < user_ids.clone().len() {
        return Ok(false)
    }
    //check user role
    if list_users.iter().any(|user| {
        println!("{}",user.role);
        !matches.contains(&user.role)
    }) {
        return Ok(false)
    } else {
        return Ok(true)
    }
}