use diesel::prelude::*;

use crate::model::department::{NewDepartment, Department, UpdateDepartment, DepartmentPayload, UpdateDepartmentPayload};
use crate::utils::validate_department_name;
use crate::handler::user::get_user;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn add_department(payload: DepartmentPayload, connection: &mut PgConnection) -> Result<Department, DbError> {
    use crate::schema::departments::dsl::*;
    if !validate_department_name(payload.department_name.clone()) {
        return Err("Validation Error".into())
    }
    let user_id = match payload.created_by {
        Some(id) => id,
        None => 0
    };

    if user_id != 0 {
        let is_exist = get_user(user_id, connection)?;
        if is_exist.is_none() {
            return Err("User Not Found".into());
        }
    }

    let new_department = NewDepartment::new(payload);

    let department = diesel::insert_into(departments)
        .values(&new_department)
        .get_result(connection)?;

    Ok(department)
}

pub fn update_department(_department_id: i32,payload: UpdateDepartmentPayload, conn: &mut PgConnection) -> Result<Department, DbError>{
    use crate::schema::departments::dsl::*;

    if !validate_department_name(payload.department_name.clone()){
        return Err("Validation Error".into())
    }

    let update_department = UpdateDepartment::new(payload);

    let department = diesel::update(departments.find(_department_id))
      .set(update_department)
      .get_result::<Department>(conn)?;
    Ok(department)
}

pub fn get_department(_department_id: i32, conn: &mut PgConnection) -> Result<Option<Department>, DbError> {
    use crate::schema::departments::dsl::*;
    
    let department = departments
      .filter(department_id.eq(_department_id))
      .first::<Department>(conn)
      .optional()?;
    Ok(department)
}  

pub fn delete_department(_department_id: i32, conn: &mut PgConnection) -> Result<Department, DbError> {
    use crate::schema::departments::dsl::*;
  
    let count = diesel::delete(departments.find(_department_id)).get_result(conn)?;
    Ok(count)
}