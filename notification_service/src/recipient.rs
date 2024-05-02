use diesel::{Queryable, Insertable, Selectable};
use serde::{Deserialize, Serialize};

use crate::schema::recipients;


#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = recipients)]
pub struct Recipient {
    pub id : i32,
    pub notification_id: i32,
    pub recipient_id: i32
} 

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = recipients)]
pub struct NewRecipient {
    pub notification_id: i32,
    pub recipient_id: i32
}