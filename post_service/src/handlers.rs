use crate::{models::{NewPost, Post, TokenClaims, PostPayload}, kafka::KafkaProducer, utils::validate_user_role};

use super::DbPool;

use actix_web::{delete, get, post, put, web, Error, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;

type DbError = Box<dyn std::error::Error + Send + Sync>;

#[post("/v1/post")]
async fn create(
  pool: web::Data<DbPool>,
  payload: web::Json<PostPayload>,
  claims: web::ReqData<TokenClaims>,
  producer: web::Data<KafkaProducer>
) -> Result<HttpResponse, Error> {
  let mut conn = pool.get().expect("Error get db pool");
  let recipents = payload.recipients.clone();
  if claims.role == 1 {
    match validate_user_role(recipents.clone(), &mut conn, vec![1,2,3]) {
      Ok(val) => if val != true {
        return Ok(HttpResponse::BadRequest().body("Fail"));
      }, 
      Err(_) => {
        return Ok(HttpResponse::InternalServerError().body("Fail"));
      }
    };
  } else if claims.role == 2 {
    match validate_user_role(recipents.clone(), &mut conn, vec![2,3]) {
      Ok(val) => if val != true {
        return Ok(HttpResponse::BadRequest().body("Fail"));
      }, 
      Err(_) => {
        return Ok(HttpResponse::InternalServerError().body("Fail"));
      }
    };
  } else {
    return Ok(HttpResponse::Unauthorized().body("Fail"));
  }
  
  let post = web::block(move || {
    let mut conn = pool.get()?;
    let user_id = claims.sub;
    add_a_post(payload.title.clone(), payload.description.clone(), user_id, &mut conn)
  })
  .await?
  .map_err(actix_web::error::ErrorInternalServerError)?;
  let message_id = Uuid::new_v4().to_string();
  let data = format!(r#"{{"message_id":"{}","user_id":"{}","event_type":"{}","post_id":"{}","title":"{}","recipient": {:?}}}"#, message_id, post.user_id.clone(), "New Post", post.id.clone(), post.title.clone(), recipents);
  producer.send_msg("posts", &data).await.expect("Error push to topic");
  Ok(HttpResponse::Ok().json(post)) 
}

#[get("/v1/post/")]
async fn index(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
  let posts = web::block(move || {
    let mut conn = pool.get()?;
    find_all(&mut conn)
  })
  .await?
  .map_err(actix_web::error::ErrorInternalServerError)?;

  Ok(HttpResponse::Ok().json(posts))
}

#[get("/v1/post/{id}")]
async fn show(id: web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
  let post = web::block(move || {
    let mut conn = pool.get()?;
    find_by_id(id.into_inner(), &mut conn)
  })
  .await?
  .map_err(actix_web::error::ErrorInternalServerError)?;

  Ok(HttpResponse::Ok().json(post))
}

#[put("/v1/post/{id}")]
async fn update(
  id: web::Path<i32>,
  payload: web::Json<PostPayload>,
  pool: web::Data<DbPool>,
  producer: web::Data<KafkaProducer>
) -> Result<HttpResponse, Error> {
  let post = web::block(move || {
    let mut conn = pool.get()?;
    update_post(id.into_inner(), payload.title.clone(), payload.description.clone(), &mut conn)
  })
  .await?
  .map_err(actix_web::error::ErrorInternalServerError)?;

  let message_id = Uuid::new_v4().to_string();
  let data = format!(r#"{{"message_id":"{}","user_id":"{}","event_type":"{}","post_id":"{}","title":"{}","recipient: {:?}}}"#, message_id,post.user_id.clone(), "Update Post", post.id.clone(), post.title.clone(), vec![2,3,4,5]);
  producer.send_msg("posts".into(), &data).await.expect("Error push to topic");
  Ok(HttpResponse::Ok().json(post)) 
}

#[delete("/v1/post/{id}")]
async fn destroy(id: web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
  let result = web::block(move || {
    let mut conn = pool.get()?;
    delete_post(id.into_inner(), &mut conn)
  })
  .await?
  .map(|post| HttpResponse::Ok().json(post))
  .map_err(actix_web::error::ErrorInternalServerError)?;

  Ok(result)
}

fn add_a_post(_title: String, _des: String, _used_id: i32, conn: &mut PgConnection) -> Result<Post, DbError> {
  use crate::schema::posts::dsl::*;

  let new_post = NewPost {
    user_id:_used_id ,
    title: _title.clone(),
    description: _des.clone()
  };

  let res = diesel::insert_into(posts)
    .values(&new_post)
    .get_result(conn)?;
  Ok(res)
}

fn find_all(conn: &mut PgConnection) -> Result<Vec<Post>, DbError> {
  use crate::schema::posts::dsl::*;

  let items = posts.load::<Post>(conn)?;
  Ok(items)
}

fn find_by_id(_id: i32, conn: &mut PgConnection) -> Result<Option<Post>, DbError> {
  use crate::schema::posts::dsl::*;

  let post = posts
    .filter(id.eq(_id))
    .first::<Post>(conn)
    .optional()?;

  Ok(post)
}

fn update_post(_id: i32, _title: String, _des: String, conn: &mut PgConnection) -> Result<Post, DbError> {
  use crate::schema::posts::dsl::*;

  let post = diesel::update(posts.find(_id))
    .set((title.eq(_title), description.eq(_des)))
    .get_result::<Post>(conn)?;
  Ok(post)
}

fn delete_post(_id: i32, conn: &mut PgConnection) -> Result<usize, DbError> {
  use crate::schema::posts::dsl::*;

  let count = diesel::delete(posts.find(_id)).execute(conn)?;
  Ok(count)
}
