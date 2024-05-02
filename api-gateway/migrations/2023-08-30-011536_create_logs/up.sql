-- Your SQL goes here
CREATE TABLE access_request (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id INT NOT NULL,
    count INT NOT NULL,
    time timestamp NOT NULL DEFAULT (now())
);
