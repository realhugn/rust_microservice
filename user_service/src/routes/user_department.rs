use actix_web::{web, Error, HttpResponse};

use crate::config::postgres::DbPool;
use crate::handler::user_department::{add_user_department, get_user_department, delete_user_department};
use crate::model::user_department::{UserDepartmentPayload, UserDepartment};

pub async fn create(pool: web::Data<DbPool>, payload: web::Json<UserDepartmentPayload>) -> Result<HttpResponse, Error> {
   let user_department: Result<UserDepartment, Error> = web::block(move || {
        let mut conn = pool.get()?;
        add_user_department(payload.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError); 

    match user_department {
        Ok(_) => Ok(HttpResponse::Ok().body("OK")),
        Err(_) => {
            return Ok(HttpResponse::BadRequest().body("FAIL"))
        }
    }
}

#[allow(non_snake_case)]
pub async fn getUserDepartment(_id : web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let _id = _id.into_inner();
    let department = web::block(move || {
        let mut conn = pool.get()?;
        get_user_department(_id, &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError);

    match department {
        Ok(department) => {
            match department {
                Some(department) => Ok(HttpResponse::Ok().json(department)),
                None => {
                    Ok(HttpResponse::NotFound().body("FAIL"))
                }
            }
        },
        Err(_) => return Ok(HttpResponse::BadRequest().body("FAIL"))
    }
}

pub async fn destroy(_id: web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let result = web::block(move || {
        let mut conn = pool.get()?;
        delete_user_department(_id.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError);

    match result {
        Ok(_) => Ok(HttpResponse::Ok().body("OK")),
        Err(_) => {
            return Ok(HttpResponse::BadRequest().body("FAIL"))
        }
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/user_department")
        .route("/", web::post().to(create))
        .route("/{id}",web::get().to(getUserDepartment))
        .route("/{id}",web::delete().to(destroy));
    conf.service(scope);
}