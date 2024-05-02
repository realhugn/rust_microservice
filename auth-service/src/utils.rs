use jsonwebtoken::{Header, encode};
use regex::Regex;
use sha256::digest;
use chrono::{Duration, DateTime, Utc};

use crate::model::{User, TokenClaims};

pub fn check_sha256_salt(password_hash : String, password_raw: String, salt: String) -> bool {
    let hash_password_raw = digest(format!("{}{}", password_raw, salt));

    password_hash == hash_password_raw
}


pub fn validate_firstname_lastname(firstname : String, lastname: String) -> bool {
    let re = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
    let caps = re.is_match(&lastname) & re.is_match(&firstname);
    caps
}

pub fn validate_email(email: String) -> bool {
    let re = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
    let caps = re.is_match(&email);
    caps
}

pub fn validate_phone(phone: String) -> bool {
    let re = Regex::new(r"^[0-9]*$").unwrap();
    let caps = re.is_match(&phone);
    caps
}

pub fn validate_password(password : String) -> bool{
    let regex_one_uppercase = Regex::new(r"[a-z]{1,}").unwrap();
    let one_uppercase = regex_one_uppercase.is_match(&password);
    let regex_one_lowercase = Regex::new(r"[A-Z]{1,}").unwrap();
    let one_lowercase = regex_one_lowercase.is_match(&password);
    let regex_one_digit = Regex::new(r"[0-9]{1,}").unwrap();
    let one_digit = regex_one_digit.is_match(&password);
    let regex_length = Regex::new(r".{8,}").unwrap();
    let length = regex_length.is_match(&password);
    let regex_one_symbol = Regex::new(r"[*@!#%&()^~{}/.,?><=+-_]{1,}").unwrap();
    let one_symbol = regex_one_symbol.is_match(&password);
    let caps = one_uppercase & one_lowercase & one_digit & length & one_symbol;
    caps
}

pub fn validate_user(username: String) -> bool {
    let re = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
    let caps = re.is_match(&username);
    caps
}

pub fn hash_sha256 (password: String , salt: String) -> String {
    digest(format!("{}{}", password, salt))
}

// fn get_env_var(var_name: &str) -> String {
//     std::env::var(var_name).unwrap_or_else(|_| panic!("{} must be set", var_name))
// }

pub fn generate_token(id: i32, role: i32, _type: String, _now : DateTime<Utc>) ->  Result<String, jsonwebtoken::errors::Error> {
    if _type == "access" {
        let iat = _now.timestamp() as usize;
        let exp = (_now + Duration::minutes(2000)).timestamp() as usize;
        let claims = TokenClaims {
            sub: id,
            role,
            iat,
            exp
        };
        let token = encode (
            &Header::new(jsonwebtoken::Algorithm::RS256),
            &claims,
            &jsonwebtoken::EncodingKey::from_rsa_pem(include_bytes!("private_key.pem"))?
        );
        token
    } else {
        let iat = _now.timestamp() as usize;
        let exp = (_now + Duration::days(30)).timestamp() as usize;
        let claims = TokenClaims {
            sub: id,
            role,
            iat,
            exp
        };
        let token = encode (
            &Header::new(jsonwebtoken::Algorithm::RS256),
            &claims,
            &jsonwebtoken::EncodingKey::from_rsa_pem(include_bytes!("private_key_refresh.pem"))?
        );
        token
    }
}

pub fn verify_jwt_token(
    token: String,
    _type: &str
) -> Result<TokenClaims, jsonwebtoken::errors::Error> {

    if _type == "access" {
        let validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256);

        let decoded = jsonwebtoken::decode::<TokenClaims>(
            token.as_str().clone(),
            &jsonwebtoken::DecodingKey::from_rsa_pem(include_bytes!("public_key.pem"))?,
            &validation,
        );
        
        match decoded {
            Ok(c) => Ok(c.claims),
            Err(e) => return Err(e.into())
        }
    } else {
        let validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256);

        let decoded = jsonwebtoken::decode::<TokenClaims>(
            token.as_str().clone(),
            &jsonwebtoken::DecodingKey::from_rsa_pem(include_bytes!("public_key_refresh.pem"))?,
            &validation,
        );
        
        match decoded {
            Ok(c) => Ok(c.claims),
            Err(e) => return Err(e.into())
        }
    }
}

#[cfg(test)]
mod test_utils {
    use actix_web::web::Bytes;
    use chrono::Utc;
    use crate::diesel::prelude::*;
    use crate::model::{UserPayload, NewUser, User};
    use crate::db::establish_connection_pool_test;
    use crate::utils::{generate_token, verify_jwt_token};

    trait BodyTest {
        fn as_str(&self) -> &str;
      }
  
      impl BodyTest for Bytes {
          fn as_str(&self) -> &str {
              std::str::from_utf8(self).unwrap()
          }
      }

    #[actix_web::test]
    async fn verify_token_test() {
        use crate::schema::users::dsl::*;
        let pool = establish_connection_pool_test();
        let user_payload = UserPayload {
            username: "successtestverify".into(),
            password: "Hung123456!".into(),
            firstname: "hung".into(),
            lastname: "nguyen".into(), 
            email: "successtestverify@abc.com".into(),
            phone: "1231231414".into(),
            status: 1,
            role: 1
        };

        let data = NewUser::new(user_payload);
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        let user: User = diesel::insert_into(users)
            .values(&data)
            .get_result(&mut conn).expect("couldn't create test user from table");
        let now = Utc::now();
        let token = generate_token(user.user_id, user.role, "access".into(), now).expect("Error");
        let claims = verify_jwt_token(token, "access").expect("Error");
        //delete after create

        diesel::delete(users.filter(username.eq("successtestverify")))
            .execute(&mut pool.get().expect("couldn't get db connection from pool"))
            .expect("couldn't delete test user from table");

        assert_eq!(
            claims.sub,
            user.user_id
        );

        assert_eq!(
            claims.role,
            1
        );
    }

    #[actix_web::test]
    async fn verify_token_fail() {
        let token = "a.jwt.access.token".to_string();
        let claims = verify_jwt_token(token, "access");
        //delete after create
        assert!(claims.is_err());
    }
}