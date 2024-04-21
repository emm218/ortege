-- Add migration script here
CREATE TABLE users(
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    PRIMARY KEY (id),
    username_hash bytea NOT NULL,
    identity_key bytea NOT NULL
);
CREATE UNIQUE INDEX ON users (username_hash);
