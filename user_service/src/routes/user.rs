use actix_web::{error, web, Error, HttpResponse};
use crate::handler::user::{add_user, get_users, get_user, delete_user, update_user, change_password,get_users_manager};
use crate::config::postgres::DbPool;
use crate::model::auth::TokenClaims;
use crate::model::user::{UserPayload, UpdateUserPayload, ChangePasswordPayload};
use crate::middleware;

pub async fn create(pool: web::Data<DbPool>, payload: web::Json<UserPayload>) -> Result<HttpResponse, Error> {
  // add user to db
  let rs = web::block(move || {
    let mut conn = pool.get()?;
    add_user(payload.into_inner(), &mut conn)
  })
  .await?
  .map_err(actix_web::error::ErrorInternalServerError);

  // response OK if user added , FAIL if some server, database or validation error occur
  match rs {
    Ok(_) => Ok(HttpResponse::Ok().body("OK")),
    Err(_) => {
        return Ok(HttpResponse::BadRequest().body("FAIL"))
    }
  }
}

#[allow(non_snake_case)]
pub async fn allUsers(pool: web::Data<DbPool>, claims: Option<web::ReqData<TokenClaims>>) -> Result<HttpResponse, Error> {
  if let Some(claims) = claims {
    let users;
    if claims.role == 1 {
      // Query list of users in database  
      users = web::block(move || {
        let mut conn = pool.get()?;
        get_users(&mut conn)
      })
      .await?
      .map_err(actix_web::error::ErrorInternalServerError);
    } else if claims.role == 2 {
      users = web::block(move || {
        let mut conn = pool.get()?;
        get_users_manager(&mut conn)
      })
      .await?
      .map_err(actix_web::error::ErrorInternalServerError);
    } else {
      return Ok(HttpResponse::BadRequest().body("FAIL"))
    }
    // response list of users or FAIL if some server, database or validation error occur
    match users {
      Ok(user) => Ok(HttpResponse::Ok().json(user)),
      Err(_) => {
          return Ok(HttpResponse::BadRequest().body("FAIL"))
      }
    }
  } else {
    return Ok(HttpResponse::Unauthorized().body("Missing token"))
  }
}

#[allow(non_snake_case)]
pub async fn getUser(user_id: web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
  // Query user info from database
  let user_id = user_id.into_inner();
  let user = web::block(move || {
  let mut conn = pool.get()?;
    get_user(user_id, &mut conn)
  })
  .await?
  .map_err(error::ErrorInternalServerError);

  // Response user information (except password and recent_password) if user found
  // Response "No user found with UID: {_id}"" if no user found
  // Response FAIL if some errors occur
  match user {
      Ok(user) => {
          match user {
              Some(user) => Ok(HttpResponse::Ok().json(user)),
              None => {
                  Ok(HttpResponse::NotFound().body(format!("No user found with UID: {}" , user_id)))
              }
          }
      },
    Err(_) => return Ok(HttpResponse::BadRequest().body("FAIL"))
  }
}

pub async fn update(_id: web::Path<i32>, payload: web::Json<UpdateUserPayload>, pool: web::Data<DbPool>,) -> Result<HttpResponse, Error> {
  // Update user info 
  let user = web::block(move || {
    let mut conn = pool.get()?;
    update_user(_id.into_inner(), payload.into_inner() , &mut conn)
  })
  .await?
  .map_err(actix_web::error::ErrorInternalServerError);

  // Response OK if user successfully updated
  // Response FAIL if some errors occur
  match user {
    Ok(_) => Ok(HttpResponse::Ok().body("OK")),
    Err(_) => {
        return Ok(HttpResponse::BadRequest().body("FAIL"))
    }
  }
}

pub async fn change_password_user(_id: web::Path<i32>, payload: web::Json<ChangePasswordPayload>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
  let user = web::block(move || {
    let mut conn = pool.get()?;
    change_password(_id.into_inner(), payload.into_inner() , &mut conn)
  })
  .await?
  .map_err(actix_web::error::ErrorInternalServerError);

  // Response OK if user successfully updated
  // Response FAIL if some errors occur
  match user {
    Ok(_) => Ok(HttpResponse::Ok().body("OK")),
    Err(_) => {
        return Ok(HttpResponse::BadRequest().body("FAIL"))
    }
  }
}

pub async fn destroy(user_id: web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
  let result = web::block(move || {
    let mut conn = pool.get()?;
    delete_user(user_id.into_inner(), &mut conn)
  })
  .await?
  .map(|user| HttpResponse::Ok().json(user))
  .map_err(actix_web::error::ErrorInternalServerError);
  match result {
    Ok(_) => Ok(HttpResponse::Ok().body("OK")),
    Err(_) =>  Ok(HttpResponse::BadRequest().body("FAIL"))
  }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/user")
        .route("/", web::post().to(create).wrap(middleware::JWTAuth))
        .route("/", web::get().to(allUsers).wrap(middleware::JWTAuth))
        .route("/{id}", web::get().to(getUser))
        .route("/{id}", web::put().to(update))
        .route("/{id}", web::delete().to(destroy))
        .route("/{id}/change_password", web::put().to(change_password_user));
    conf.service(scope);
}
