use actix_web::{web, HttpResponse, Error};

use crate::{db::DbPool, model::SignPayload, handler::sign_key};

pub async fn sign(pool: web::Data<DbPool>, payload: web::Json<SignPayload>) -> Result<HttpResponse, Error> {
    // add user to db
    let rs = web::block(move || {
      let mut conn = pool.get()?;
      sign_key(payload.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError);
  
    // response OK if user added , FAIL if some server, database or validation error occur
    match rs {
      Ok(rs) => Ok(HttpResponse::Ok().body(rs)),
      Err(_) => {
          return Ok(HttpResponse::BadRequest().body("FAIL"))
      }
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/genkey")
        .route("/sign", web::post().to(sign));
    conf.service(scope);
}