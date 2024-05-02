use chrono::Utc;
use diesel::prelude::*;
use crate::model::user::{User, NewUser, UpdateUserPayload, UserPayload, UpdateUser, ChangePasswordPayload};
use crate::utils::{validate_firstname_lastname, validate_email, validate_phone, validate_user, validate_password, check_sha256_salt, hash_sha256};

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn add_user(payload: UserPayload, connection: &mut PgConnection) -> Result<User, DbError> {
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

pub fn get_users(conn: &mut PgConnection) -> Result<Vec<User>, DbError>{
    use crate::schema::users::dsl::*;
    
    let all_users = users.load::<User>(conn)?;
    Ok(all_users)
}

pub fn get_users_manager (conn: &mut PgConnection) -> Result<Vec<User>, DbError> {
    use crate::schema::users::dsl::*;
    
    let all_users = users
        .filter(role.eq(2))
        .or_filter(role.eq(3))
        .load::<User>(conn)?;
    Ok(all_users)
}

pub fn update_user(_user_id: i32,payload: UpdateUserPayload, conn: &mut PgConnection) -> Result<User, DbError>{
    let is_ok = validate_email(payload.email.clone()) 
        & validate_firstname_lastname(payload.firstname.clone(), payload.lastname.clone())
        & validate_phone(payload.phone.clone());
    if !is_ok {
        return Err("Validation Error".into());
    }

    let data = UpdateUser::new(payload);

    use crate::schema::users::dsl::*;
    let user = diesel::update(users.find(_user_id))
      .set(data)
      .get_result::<User>(conn)?;
    Ok(user)
}

pub fn get_user(_user_id: i32, conn: &mut PgConnection) -> Result<Option<User>, DbError> {
    use crate::schema::users::dsl::*;
  
    let user = users
      .filter(user_id.eq(_user_id))
      .first::<User>(conn)
      .optional()?;
    Ok(user)
}  

pub fn change_password(_user_id:i32, payload : ChangePasswordPayload, conn: &mut PgConnection) -> Result<User, DbError> {
    use crate::schema::users::dsl::*;
    let user = get_user(_user_id, conn)?;
    let user = match user {
        Some(a) => a,
        None => return Err("User Not Found".into())
    };

    let (
        db_recent_password, 
        user_salt, 
        db_password, 
        old_password_input,
        new_password_input
    ) = (
        user.recent_password, 
        user.salt, 
        user.password, 
        payload.old_password,
        payload.password
    );

    if !validate_password(new_password_input.clone()) {
        return Err("Validation Fail".into())
    }

    if !check_sha256_salt(db_password.clone(), old_password_input, user_salt.clone()) {
        return Err("Wrong old password".into())
    }

    match db_recent_password {
        Some(db_recent_password) => {
            if check_sha256_salt(db_recent_password, new_password_input.clone(), user_salt.clone()) {
                return Err("Fail".into())
            } else {
                let hash_new_password = hash_sha256(new_password_input, user_salt.clone());
                let user = diesel::update(users.find(_user_id))
                    .set((
                        password.eq(hash_new_password), 
                        recent_password.eq(db_password), 
                        updated_at.eq(Utc::now().naive_utc())
                    ))
                    .get_result::<User>(conn)?;
                Ok(user)
            }
        },
        None => {
            let hash_new_password = hash_sha256(new_password_input, user_salt.clone());
            let user = diesel::update(users.find(_user_id))
                .set((
                    password.eq(hash_new_password), 
                    recent_password.eq(db_password), 
                    updated_at.eq(Utc::now().naive_utc())
                ))
                .get_result::<User>(conn)?;
            Ok(user)
        }
    }
}

pub fn delete_user(_user_id: i32, conn: &mut PgConnection) -> Result<User, DbError> {
    use crate::schema::users::dsl::*;
  
    let count = diesel::delete(users.find(_user_id)).get_result(conn)?;
    Ok(count)
}