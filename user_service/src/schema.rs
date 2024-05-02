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
    group_user (id) {
        id -> Int4,
        user_id -> Int4,
        group_id -> Int4,
    }
}

diesel::table! {
    groups (id) {
        id -> Int4,
        name -> Text,
        role -> Int4,
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
        entity_id -> Text,
        recipient -> Int4,
        created_at -> Timestamp,
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
    recipients (id) {
        id -> Int4,
        notification_id -> Int4,
        recipient_id -> Int4,
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
    group_user,
    groups,
    notifications,
    posts,
    recipients,
    sessions,
    user_department,
    users,
);
