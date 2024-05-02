use chrono::Utc;
use diesel::PgConnection;
use diesel::prelude::*;

use crate::model::AccessKey;
use crate::{model::{SignPayload, NewAccessKey}, utils::generate_token};

type Error = Box <dyn std::error::Error + Send + Sync>;

pub fn sign_key(payload: SignPayload, connection: &mut PgConnection) -> Result<String, Error> {
    use crate::schema::access_key::dsl::*;
    let now = Utc::now();
    let access_token = generate_token(payload.uid.clone(), payload.allow, payload.exp, now).expect("Error generating token");
    let new_access_key = NewAccessKey {
        token: access_token.clone(),
        permission: payload.permission
    };
    let _ = diesel::insert_into(access_key).values(&new_access_key).load::<AccessKey>(connection);
    Ok(access_token)
}
