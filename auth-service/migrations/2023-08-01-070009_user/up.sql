-- Your SQL goes here
CREATE TABLE users (
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

CREATE TABLE departments (
    department_id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    department_name TEXT NOT NULL UNIQUE,
    created_by INT,
    created_at timestamp NOT NULL DEFAULT (now()),
    updated_at timestamp NOT NULL DEFAULT (now()),
    status INT NOT NULL
);

CREATE TABLE user_department (
    ud_id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id INT NOT NULL,
    department_id INT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE,
    FOREIGN KEY (department_id) REFERENCES departments(department_id) ON DELETE CASCADE,
    UNIQUE (user_id, department_id)
);

CREATE TABLE sessions (
    session_id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id INT NOT NULL,
    role INT NOT NULL,
    expired_date timestamp NOT NULL,
    token TEXT NOT NULL
);