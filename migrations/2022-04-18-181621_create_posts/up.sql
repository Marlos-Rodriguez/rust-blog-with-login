-- Your SQL goes here
CREATE TABLE posts (
  id VARCHAR(36) PRIMARY KEY,
  title VARCHAR NOT NULL,
  slug VARCHAR NOT NULL UNIQUE,
  author_id VARCHAR(36) NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f',
  created_at timestamp not null default current_timestamp,
  updated_at timestamp not null default current_timestamp,
  foreign key (author_id) references users(id)
);