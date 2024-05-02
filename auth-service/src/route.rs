use actix_web::{web, Error, HttpResponse, Responder, HttpRequest};
use chrono::Utc;
use serde_json::json;
use crate::db::DbPool;
use crate::handler::{register, login, refresh_token, delete_session, list_sessions};
use crate::middleware;
use crate::model::{UserPayload, LoginPayload};
use crate::utils::{generate_token, verify_jwt_token};

pub async fn create(pool: web::Data<DbPool>, payload: web::Json<UserPayload>) -> Result<HttpResponse, Error> {
    // add user to db
    let rs = web::block(move || {
      let mut conn = pool.get()?;
      register(payload.into_inner(), &mut conn)
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

pub async fn sign_in(pool: web::Data<DbPool>, payload: web::Json<LoginPayload>) -> Result<HttpResponse, Error> {
    let rs = web::block(move || {
        let mut conn = pool.get()?;
        login(payload.into_inner(), &mut conn)
      })
      .await?
      .map_err(actix_web::error::ErrorInternalServerError);
    
      // response OK if user added , FAIL if some server, database or validation error occur
      match rs {
        Ok(token) => Ok(HttpResponse::Ok().json(json!({"access token" : token.0, "refresh_token": token.1}))),
        Err(_) => {
            return Ok(HttpResponse::BadRequest().body("FAIL"))
        }
      }
}

pub async fn refresh(pool: web::Data<DbPool>, req: HttpRequest) -> Result<HttpResponse, Error>{
  let message = "could not refresh access token";
  let rf_token = match req.cookie("refresh_token") {
    Some(c) => c.value().to_string(),
    None => {
      return Ok(HttpResponse::Forbidden().json(serde_json::json!({"status": "fail", "message": message})));
    }
  };

  let verify = verify_jwt_token(rf_token.clone(), "refresh");

  if let Err(_) = verify {
    return Ok(HttpResponse::Forbidden().json(serde_json::json!({"status": "fail", "message": "fail"})));
  } 

  let refresh_token_check = web::block(move || {
    let mut conn = pool.get()?;
    refresh_token(rf_token, &mut conn)
  }).await?
  .map_err(actix_web::error::ErrorInternalServerError);
  let now = Utc::now().naive_utc();
  match refresh_token_check {
    Ok(token) => {
      match token {
        Some(token) => {
          if !token.expired_date.gt(&now) {
            return Ok(HttpResponse::Unauthorized().body("Refresh Token Expired"));
          } else {
            let now = Utc::now();
            Ok(HttpResponse::Ok().body(generate_token(token.user_id, token.role, "access".into(), now).unwrap()))
          }
        }, 
        None => Ok(HttpResponse::Unauthorized().body("Unauthorized"))
      }
    },
    Err(_) => return Ok(HttpResponse::BadRequest().body("Error"))
  }
}

pub async fn delete(pool: web::Data<DbPool>, token: web::Path<String>) -> Result<HttpResponse, Error> {
  let result = web::block(move || {
    let mut conn = pool.get()?;
    delete_session(token.into_inner(), &mut conn)
  })
  .await?
  .map_err(actix_web::error::ErrorInternalServerError);
  match result {
    Ok(_) => Ok(HttpResponse::Ok().body("OK")),
    Err(_) =>  Ok(HttpResponse::BadRequest().body("FAIL"))
  }
}

pub async fn list_all_session(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
  let result = web::block(move || {
    let mut conn = pool.get()?;
    list_sessions(&mut conn)
  })
  .await?
  .map_err(actix_web::error::ErrorInternalServerError);
  match result {
    Ok(result) => Ok(HttpResponse::Ok().json(result)),
    Err(_) =>  Ok(HttpResponse::BadRequest().body("FAIL"))
  }
}

async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Hello from auth service";

    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": MESSAGE}))
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/auth")
        .route("/register", web::post().to(create))
        .route("/signin", web::post().to(sign_in))
        .route("/refresh", web::get().to(refresh))
        .route("/delete/{token}", web::delete().to(delete).wrap(middleware::JWTAuth))
        .route("/sessions/", web::get().to(list_all_session).wrap(middleware::JWTAuth))
        .route("/health_check", web::get().to(health_checker_handler));
    conf.service(scope);
}