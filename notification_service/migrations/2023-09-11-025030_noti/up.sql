CREATE TABLE IF NOT EXISTS notifications (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id INT NOT NULL,
    description TEXT NOT NULL,
    title TEXT NOT NULL,
    type INT NOT NULL,
    entity_id TEXT NOT NULL,
    created_at timestamp NOT NULL DEFAULT (now())
);

CREATE TABLE IF NOT EXISTS recipients (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    notification_id INT NOT NULL,
    recipient_id INT NOT NULL
);