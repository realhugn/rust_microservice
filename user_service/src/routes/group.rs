use actix_web::{web, HttpResponse, Error};

use crate::config::postgres::DbPool; 
use crate::handler::group::{create_group, create_user_group, get_all_groups, get_user_groups, get_group_users};
use crate::middleware;
use crate::model::group::{NewGroup, NewGroupUser};

pub async fn create(pool: web::Data<DbPool>, payload: web::Json<NewGroup>) -> Result<HttpResponse, Error> {
    let rs = web::block(move || {
        let mut conn = pool.get()?;
        create_group(payload.into_inner(), &mut conn)
    })
    .await
    .map_err(actix_web::error::ErrorInternalServerError);

    match rs {
        Ok(_) => Ok(HttpResponse::Ok().body("OK")),
        Err(_) => {
            return Ok(HttpResponse::BadRequest().body("FAIL"))
        }
    }
}

pub async fn create_u_group(pool: web::Data<DbPool>, payload: web::Json<NewGroupUser>) -> Result<HttpResponse, Error> {
    let rs = web::block(move || {
        let mut conn = pool.get()?;
        create_user_group(payload.into_inner(), &mut conn)
    })
    .await
    .map_err(actix_web::error::ErrorInternalServerError);

    match rs {
        Ok(_) => Ok(HttpResponse::Ok().body("OK")),
        Err(_) => {
            return Ok(HttpResponse::BadRequest().body("FAIL"))
        }
    }
}

pub async fn all_groups(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let groups = web::block(move || {
        let mut conn = pool.get()?;
        get_all_groups(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError);

    match groups {
        Ok(group) => Ok(HttpResponse::Ok().json(group)),
        Err(_) => {
            return Ok(HttpResponse::BadRequest().body("FAIL"))
        }
      }
}

pub async fn user_groups(user_id: web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let groups = web::block(move || {
        let mut conn = pool.get()?;
        get_user_groups(user_id.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError);

    match groups {
        Ok(group) => Ok(HttpResponse::Ok().json(group)),
        Err(_) => {
            return Ok(HttpResponse::BadRequest().body("FAIL"))
        }
      }
}

pub async fn group_users(group_id: web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let users = web::block(move || {
        let mut conn = pool.get()?;
        get_group_users(group_id.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError);

    match users {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(_) => {
            return Ok(HttpResponse::BadRequest().body("FAIL"))
        }
      }
}



pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/group")
        .route("/", web::post().to(create).wrap(middleware::JWTAuth))
        .route("/user/", web::post().to(create_u_group))
        .route("/", web::get().to(all_groups))
        .route("/user/{id}", web::get().to(user_groups))
        .route("/{id}", web::get().to(group_users));
    conf.service(scope);
}