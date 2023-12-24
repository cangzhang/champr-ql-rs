-- Your SQL goes here
CREATE TABLE logs (
    id SERIAL PRIMARY KEY,
    action TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
)