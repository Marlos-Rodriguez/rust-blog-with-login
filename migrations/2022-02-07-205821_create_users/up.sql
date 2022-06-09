-- Your SQL goes 
create table users (
    id VARCHAR(36) PRIMARY KEY,
    username VARCHAR not null unique,
    email VARCHAR not null unique,
    password VARCHAR not null,
    is_admin BOOLEAN NOT NULL DEFAULT 'f',
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);