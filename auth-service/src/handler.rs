use chrono::Duration;
use chrono::Utc;
use diesel::PgConnection;
use diesel::prelude::*;
use crate::model::NewSession;
use crate::model::{NewUser, UserPayload, LoginPayload, User, Session};
use crate::utils::{check_sha256_salt, validate_email, validate_firstname_lastname, validate_password, validate_phone, validate_user, generate_token};

type DbError = Box <dyn std::error::Error + Send + Sync>;

pub fn login(payload : LoginPayload, connection: &mut PgConnection) -> Result<(String, String), DbError> {
    use crate::schema::users::dsl::*;
    use crate::schema::sessions::dsl::*;

    let (_username, _password) = (payload.username.clone(), payload.password.clone());
    let user = users.filter(username.eq(_username)).first::<User>(connection).optional()?;
    match user {
        None => return Err("User not exist".into()),
        Some(user) => {
            let (user_salt, db_password) = (user.salt.clone(), user.password.clone());
            if !check_sha256_salt(db_password, _password, user_salt) {
                return Err("Password do not match".into())
            }
            let now = Utc::now();
            let access_token = generate_token(user.user_id.clone(), user.role.clone(), "access".into(), now)?;
            let refresh_token = generate_token(user.user_id.clone(), user.role.clone(), "refresh".into(), now)?;   
            let new_session = NewSession {
                user_id: user.user_id,
                role: user.role,
                expired_date: (now + Duration::days(30)).naive_utc(),
                token: refresh_token.clone(),
            };
            let _ = diesel::insert_into(sessions).values(&new_session).load::<Session>(connection);
            Ok((access_token, refresh_token))
        }
    }
}

pub fn register(payload: UserPayload, connection: &mut PgConnection) -> Result<User, DbError> {
    let is_ok = validate_firstname_lastname(payload.firstname.clone(),payload.lastname.clone())
        & validate_email(payload.email.clone())
        & validate_phone(payload.phone.clone())
        & validate_user(payload.username.clone())
        & validate_password(payload.password.clone());
    
    if !is_ok {
        return Err("Validation Error".into());
    }

    let new_user = NewUser::new(payload);

    use crate::schema::users::dsl::*;   
    let user = diesel::insert_into(users)
        .values(&new_user)
        .get_result(connection)?;

    Ok(user)
}

pub fn refresh_token(_token: String, connection: &mut PgConnection) -> Result<Option<Session>, DbError> {
    use crate::schema::sessions::dsl::*;  
    let db_sessions = sessions.filter(token.eq(_token)).first::<Session>(connection).optional()?; 
    Ok(db_sessions)
} 

pub fn delete_session(_token: String, connection: &mut PgConnection) -> Result<Session, DbError> {
    use crate::schema::sessions::dsl::*;
    let count = diesel::delete(sessions.filter(token.eq(_token))).get_result(connection)?;
    Ok(count)
}

pub fn list_sessions(connection: &mut PgConnection) -> Result<Vec<Session>, DbError> {
    use crate::schema::sessions::dsl::*;
    let _sessions = sessions.load::<Session>(connection)?;
    Ok(_sessions)
}
