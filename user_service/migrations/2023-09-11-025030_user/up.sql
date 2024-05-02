-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users (
    user_id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    username TEXT NOT NULL UNIQUE, 
    password TEXT NOT NULL,
    recent_password TEXT DEFAULT NULL,
    firstname TEXT NOT NULL,
    lastname TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    phone TEXT NOT NULL,
    created_at timestamp NOT NULL DEFAULT (now()), 
    updated_at timestamp NOT NULL DEFAULT (now()), 
    status INT NOT NULL,
    salt TEXT NOT NULL,
    role INT NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS departments (
    department_id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    department_name TEXT NOT NULL UNIQUE,
    created_by INT,
    created_at timestamp NOT NULL DEFAULT (now()),
    updated_at timestamp NOT NULL DEFAULT (now()),
    status INT NOT NULL
);

CREATE TABLE IF NOT EXISTS user_department (
    ud_id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id INT NOT NULL,
    department_id INT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE,
    FOREIGN KEY (department_id) REFERENCES departments(department_id) ON DELETE CASCADE,
    UNIQUE (user_id, department_id)
);

CREATE TABLE IF NOT EXISTS notifications (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id INT NOT NULL,
    description TEXT NOT NULL,
    title TEXT NOT NULL,
    type INT NOT NULL,
    entity_id TEXT NOT NULL,
    recipient INT NOT NULL, 
    created_at timestamp NOT NULL DEFAULT (now())
);

CREATE TABLE IF NOT EXISTS recipients (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    notification_id INT NOT NULL,
    recipient_id INT NOT NULL
);

CREATE TABLE IF NOT EXISTS posts (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id INT NOT NULL DEFAULT 0, 
    title TEXT NOT NULL,
    description TEXT NOT NULL, 
    created_at timestamp NOT NULL DEFAULT (now()),
    updated_at timestamp NOT NULL DEFAULT (now())
);

CREATE TABLE IF NOT EXISTS groups (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name TEXT NOT NULL,
    role INT NOT NULL
);

CREATE TABLE IF NOT EXISTS group_user ( 
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id INT NOT NULL,
    group_id INT NOT NULL
);