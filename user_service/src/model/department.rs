use chrono::Utc;
use diesel::{Queryable, Insertable, AsChangeset, Identifiable, Selectable, Associations};
use serde::{Deserialize, Serialize};

use crate::schema::departments;

use super::user::User;

#[derive(Debug, Serialize, Deserialize, Queryable, Associations, Selectable, Identifiable, PartialEq)]
#[diesel(belongs_to(User, foreign_key = created_by))]
#[diesel(primary_key(department_id))]
pub struct Department {
    pub department_id : i32,
    pub department_name: String,
    pub created_by :Option<i32>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub status: i32,
}

#[derive(Debug, Serialize, Deserialize,Insertable, Clone)]
#[diesel(table_name = departments)]
pub struct NewDepartment{
    pub department_name: String,
    pub created_by : i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub status: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset)]
#[diesel(table_name = departments)]
pub struct UpdateDepartment{
    pub department_name: String,
    pub updated_at: chrono::NaiveDateTime,
    pub status: i32,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateDepartmentPayload {
    pub department_name: String,
    pub status: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DepartmentPayload {
    pub department_name: String,
    pub created_by : Option<i32>,
    pub status: i32,
}

impl NewDepartment {
    pub fn new(payload: DepartmentPayload) -> Self {
        let now = Utc::now().naive_utc();
        let user_id = match payload.created_by {
            Some(id) => id,
            None => 0
        };
        Self { 
            department_name: (payload.department_name), 
            created_by: (user_id), 
            created_at: (now), 
            updated_at: (now), 
            status: (payload.status) 
        }
    }
}

impl UpdateDepartment {
    pub fn new(payload: UpdateDepartmentPayload) -> Self {
        let now = Utc::now().naive_utc();
        Self { 
            department_name: (payload.department_name), 
            updated_at: (now), 
            status: (payload.status) 
        }
    }
}