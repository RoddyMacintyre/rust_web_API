-- Your SQL goes here
-- UP means setting up, DOWN means breaking down
CREATE TABLE exchanges (
                           id INTEGER PRIMARY KEY AUTOINCREMENT,
                           name VARCHAR NOT NULL,
                           url VARCHAR NOT NULL,
                           created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)