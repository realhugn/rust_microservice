use diesel::prelude::*;

use crate::model::{group::{NewGroup, Group, NewGroupUser, GroupUser}, user::User};

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn create_group(payload: NewGroup, connection: &mut PgConnection) -> Result<Group, DbError> {
    use crate::schema::groups::dsl::*;
    let group = diesel::insert_into(groups)
        .values(&payload)
        .get_result(connection)?;

    Ok(group)
}

pub fn create_user_group(payload: NewGroupUser, connection: &mut PgConnection) -> Result<GroupUser, DbError> {
    use crate::schema::group_user::dsl::*;
    let new_user_group = diesel::insert_into(group_user)
        .values(&payload)
        .get_result(connection)?;
    Ok(new_user_group)
}

pub fn get_all_groups(connection: &mut PgConnection) -> Result<Vec<Group>,DbError> {
    use crate::schema::groups::dsl::*;

    let all_groups = groups.load::<Group>(connection)?;
    Ok(all_groups)
}

pub fn get_user_groups (_user_id: i32, connection : &mut PgConnection) -> Result<Vec<Group>, DbError> {
    use crate::schema::group_user;
    use crate::schema::groups;

    let group_id = group_user::table.filter(group_user::user_id.eq(_user_id)).select(group_user::group_id).load::<i32>(connection)?;
    let group = groups::table.filter(groups::id.eq_any(group_id)).load::<Group>(connection)?;
    Ok(group)
}

pub fn get_group_users (_group_id: i32, connection : &mut PgConnection) -> Result<Vec<User>, DbError> {
    use crate::schema::group_user;
    use crate::schema::users;

    let user_ids = group_user::table.filter(group_user::group_id.eq(_group_id)).select(group_user::user_id).load::<i32>(connection)?;
    let group = users::table.filter(users::user_id.eq_any(user_ids)).load::<User>(connection)?;
    Ok(group)
}