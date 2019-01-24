-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE users (
    id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
    name TEXT NOT NULL,
    login_name TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    email TEXT NOT NULL
);

CREATE TABLE user_tokens (
    id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
    user_id UUID NOT NULL REFERENCES users(id),
    created_on TIMESTAMPTZ NOT NULL,
    ip TEXT NOT NULL
);

CREATE TABLE request_logs (
    id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
    url TEXT NOT NULL,
    headers TEXT NOT NULL,
    response_code INT NULL,
    response_size_bytes INT NULL,
    created_on TIMESTAMPTZ NOT NULL,
    finished_on TIMESTAMPTZ NULL
);
