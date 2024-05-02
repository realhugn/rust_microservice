use actix_web::{web, Error, HttpResponse};
use crate::config::postgres::DbPool;
use crate::handler::department::{add_department, get_department, delete_department, update_department};
use crate::model::department::{DepartmentPayload, Department, UpdateDepartmentPayload};

pub async fn create(pool: web::Data<DbPool>, payload: web::Json<DepartmentPayload>) -> Result<HttpResponse, Error> {

   let department: Result<Department, Error> = web::block(move || {
        let mut conn = pool.get()?;
        add_department(payload.into_inner(), &mut conn)
   })
   .await?
   .map_err(actix_web::error::ErrorInternalServerError); 

    match department {
        Ok(_) => Ok(HttpResponse::Ok().body("OK")),
        Err(_) => {
            return Ok(HttpResponse::BadRequest().body("FAIL"))
        }
    }
}

#[allow(non_snake_case)]
pub async fn getDepartment(_id : web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let _id = _id.into_inner();
    let department = web::block(move || {
        let mut conn = pool.get()?;
        get_department(_id, &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError);

    match department {
        Ok(department) => {
            match department {
                Some(department) => Ok(HttpResponse::Ok().json(department)),
                None => {
                    Ok(HttpResponse::NotFound().body(format!("No user found with UID: {}" , _id)))
                }
            }
        },
        Err(_) => return Ok(HttpResponse::BadRequest().body("FAIL"))
    }
}

pub async fn update(
  _id: web::Path<i32>,
  payload: web::Json<UpdateDepartmentPayload>,
  pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let _id = _id.into_inner();

    let updated_department = web::block(move || {
        let mut conn = pool.get()?;
        update_department(_id, payload.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError);

    match updated_department {
        Ok(_) => Ok(HttpResponse::Ok().body("OK")),
        Err(_) => {
            return Ok(HttpResponse::BadRequest().body("FAIL"))
        }
    }
}

pub async fn destroy(_id: web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let result = web::block(move || {
        let mut conn = pool.get()?;
        delete_department(_id.into_inner(), &mut conn)
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
    let scope = web::scope("/department")
        .route("/", web::post().to(create))
        .route("/{id}", web::put().to(update))
        .route("/{id}",web::get().to(getDepartment))
        .route("/{id}",web::delete().to(destroy));
    conf.service(scope);
}