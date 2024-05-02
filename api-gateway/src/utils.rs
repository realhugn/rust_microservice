use diesel::prelude::*;
use jsonwebtoken::TokenData;

use crate::{model::TokenClaims, config::DbPool};

pub fn verify_jwt_token(
    token: &str,
) -> Result<TokenData<TokenClaims>, jsonwebtoken::errors::Error> {

    let validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256);

    let decoded = jsonwebtoken::decode::<TokenClaims>(
        token,
        &jsonwebtoken::DecodingKey::from_rsa_pem(include_bytes!("public_key.pem"))?,
        &validation,
    );

    decoded
}

pub struct Rule {
    // pub method: String,
    pub endpoint: String,
    pub group_role: Vec<i32>
}

pub fn authorize_request_group(pool: DbPool, path: String, _user_id: i32) -> bool {
    use crate::schema::group_user::dsl::*;
    let rules = vec![
        Rule {
            // method: "GET".into(),
            endpoint: "/v1/alert/wazuh/".into(),
            group_role: vec![1,2,3,5]
        },
        Rule {
            // method: "GET".into(),
            endpoint: "/v1/alert/thehive/".into(),
            group_role: vec![4,6]
        },
        Rule {
            endpoint: "/v1/notification".into(),
            group_role: vec![0]
        },
        Rule {
            endpoint: "/v1/post".into(),
            group_role: vec![0]
        },
        Rule {
            endpoint: "/v1/wazuh".into(),
            group_role: vec![0]
        },
        Rule {
            endpoint: "/v1/user".into(),
            group_role: vec![0]
        },
        Rule {
            endpoint: "/v1/group".into(),
            group_role: vec![0]
        }
    ];

    let mut conn = pool.get().expect("Error get db");
    let group_role = group_user.filter(user_id.eq(_user_id)).select(group_id).first::<i32>(&mut conn);
    let group_role = match group_role {
        Ok(role) => role,
        Err(_) => 0
    };

    let mut rule_idx = 0;
    for i in 0..rules.len() {
        if path.starts_with(&rules[i].endpoint) {
            rule_idx = i
        }
    }

    if rules[rule_idx].group_role[0] == 0 {
        return true
    } else {
        if rules[rule_idx].group_role.contains(&group_role) {
            return true
        }
        return false
    }
    
    // if rules.iter().any(|x| {
    //     println!("{:?} {:?} ", path.starts_with(&x.endpoint), x.group_role.contains(&group_role));
    //     path.starts_with(&x.endpoint) && x.group_role.contains(&group_role)
    // }) {
    //     return true
    // } else {
    //     return false
    // }
}