use diesel::prelude::*;

use crate::model::user_department::{NewUserDepartment, UserDepartment, UsersInDepartment, UserDepartmentPayload};
use crate::model::user::User;
use crate::schema::{departments, users};

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn add_user_department(payload: UserDepartmentPayload, connection: &mut PgConnection) -> Result<UserDepartment, DbError> {
    use crate::schema::user_department::dsl::*;
    let data = NewUserDepartment::new(payload);
    let record = diesel::insert_into(user_department)
        .values(&data)
        .get_result(connection)?;

    Ok(record)
}

pub fn get_user_department(_user_department_id: i32, conn: &mut PgConnection) -> Result<Option<UserDepartment>, DbError> {
    use crate::schema::user_department::dsl::*;
  
    let rs = user_department
      .filter(ud_id.eq(_user_department_id))
      .first::<UserDepartment>(conn)
      .optional()?;
    Ok(rs)
}  

#[allow(dead_code)]
pub fn get_user_in_department(_department_id: i32, conn: &mut PgConnection) -> Result<UsersInDepartment, DbError> {
    use crate::schema::user_department;

    let users = user_department::table
        .inner_join(departments::table)
        .inner_join(users::table)
        .select(User::as_select())
        .filter(departments::department_id.eq(_department_id))
        .load::<User>(conn)?;
    let result = UsersInDepartment {
        department_id: _department_id,
        users
    };
    
    Ok(result)
}  

pub fn delete_user_department(_user_department_id: i32, conn: &mut PgConnection) -> Result<UserDepartment, DbError> {
    use crate::schema::user_department::dsl::*;
  
    let count = diesel::delete(user_department.find(_user_department_id)).get_result(conn)?;
    Ok(count)
}