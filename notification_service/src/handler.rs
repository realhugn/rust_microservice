use actix_web::HttpResponse;
use actix_web::get;
use actix_web::Error;
use actix_web::web;
use diesel::PgConnection;
use diesel::prelude::*;

use crate::DbPool;
use crate::model::Notification;
use crate::model::TokenClaims;
use crate::recipient::NewRecipient;
use crate::recipient::Recipient;
use crate::{kafka::KafkaMessage, model::NewNotification};

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn create(payload: KafkaMessage, connection: &mut PgConnection) -> Result<Notification, DbError> {
    use crate::schema::notifications::dsl::*;
    let i;
    if payload.event_type == "New Post" {
        i = 1;
    } else if payload.event_type == "Update Post" {
        i = 2;
    } else {
        i = 3;
    }


    let new_post = NewNotification{
        user_id: payload.user_id.parse::<i32>().unwrap(),
        description: "test".into(),
        title: payload.title,
        type_: i,
        entity_id: payload.post_id
    };

    let post: Notification = diesel::insert_into(notifications)
        .values(&new_post)
        .get_result::<Notification>(connection)?;

    for i in 0..payload.recipient.len() {
        let new_recipient = NewRecipient {
            notification_id: post.id,
            recipient_id: payload.recipient[i],
        };
        let _: Recipient = diesel::insert_into(crate::schema::recipients::table)
        .values(&new_recipient)
        .get_result::<Recipient>(connection)?;
    }
    Ok(post)
}

#[get("/")]
async fn index(pool: web::Data<DbPool>, claims: web::ReqData<TokenClaims>) -> Result<HttpResponse, Error> {
    
    let user_id = claims.sub;

    let posts = web::block(move || {
        let mut conn = pool.get()?;
        find_all(&mut conn, user_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(posts))
}

fn find_all(conn: &mut PgConnection, _user_id : i32) -> Result<Vec<Notification>, DbError> {
    use crate::schema::notifications::dsl::*;
    use crate::schema::recipients;

    let ids: Vec<i32> = recipients::table.select(recipients::notification_id).filter(recipients::recipient_id.eq(_user_id)).load::<i32>(conn)?;

    let items = notifications.filter(id.eq_any(ids)).load::<Notification>(conn)?;
    Ok(items)
}

#[get("/heath_check")]
async fn heath_check() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Ok"))
}