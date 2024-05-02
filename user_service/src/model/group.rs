use diesel::{Queryable, Insertable, Identifiable, Selectable};
use serde::{Deserialize, Serialize};
use crate::schema::{groups, group_user};

#[derive(Debug, Serialize, Deserialize,Queryable, Selectable, Identifiable, PartialEq,Clone)]
pub struct Group {
    pub id : i32,
    pub name: String,
    pub role: i32
}

#[derive(Debug, Serialize, Deserialize,Insertable, Clone)]
#[diesel(table_name = groups)]
pub struct NewGroup {
    pub name: String,
    pub role: i32
}

#[derive(Debug, Serialize, Deserialize,Queryable, Selectable, Identifiable, PartialEq,Clone)]
#[diesel(table_name = group_user)]
pub struct GroupUser {
    pub id: i32,
    pub user_id: i32,
    pub group_id: i32
}

#[derive(Debug, Serialize, Deserialize,Insertable, Clone)]
#[diesel(table_name = group_user)]
pub struct NewGroupUser {
    pub user_id: i32,
    pub group_id: i32
}