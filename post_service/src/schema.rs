// @generated automatically by Diesel CLI.

diesel::table! {
    departments (department_id) {
        department_id -> Int4,
        department_name -> Text,
        created_by -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        status -> Int4,
    }
}

diesel::table! {
    notifications (id) {
        id -> Int4,
        user_id -> Int4,
        description -> Text,
        title -> Text,
        #[sql_name = "type"]
        type_ -> Int4,
        entity_id -> Int4,
        time -> Timestamp,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        user_id -> Int4,
        title -> Text,
        description -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    sessions (session_id) {
        session_id -> Int4,
        user_id -> Int4,
        role -> Int4,
        expired_date -> Timestamp,
        token -> Text,
    }
}

diesel::table! {
    user_department (ud_id) {
        ud_id -> Int4,
        user_id -> Int4,
        department_id -> Int4,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        username -> Text,
        password -> Text,
        recent_password -> Nullable<Text>,
        firstname -> Text,
        lastname -> Text,
        email -> Text,
        phone -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        status -> Int4,
        salt -> Text,
        role -> Int4,
    }
}

diesel::joinable!(user_department -> departments (department_id));
diesel::joinable!(user_department -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    departments,
    notifications,
    posts,
    sessions,
    user_department,
    users,
);
