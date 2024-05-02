use diesel::{Queryable, Insertable, AsChangeset, Identifiable, Selectable, Associations};
use serde::{Deserialize, Serialize};

use crate::schema::user_department;

use super::user::User;
use super::department::Department;

#[derive(Debug, Serialize, Deserialize, Queryable, Associations, Selectable, Identifiable, PartialEq)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(belongs_to(Department, foreign_key = department_id))]
#[diesel(primary_key(ud_id))]
#[diesel(table_name = user_department)]
pub struct UserDepartment {
    pub ud_id : i32, 
    pub user_id: i32,
    pub department_id : i32,
}

#[derive(Debug, Serialize, Deserialize, Insertable, Clone, AsChangeset)]
#[diesel(table_name = user_department)]
pub struct NewUserDepartment{
    pub user_id: i32,
    pub department_id : i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserDepartmentPayload {
    pub user_id :i32,
    pub department_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UsersInDepartment {
    pub department_id: i32,
    pub users : Vec<User>
}

impl NewUserDepartment {
    pub fn new(payload : UserDepartmentPayload) -> Self {
        Self { 
            user_id: (payload.user_id),
            department_id: (payload.department_id) 
        }
    }
}

