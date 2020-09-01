-- Your SQL goes here
CREATE TABLE todos (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  completed BOOLEAN NOT NULL DEFAULT 'f',
  inserted_at timestamp NOT NULL DEFAULT (now() at time zone 'utc'),
  updated_at timestamp NOT NULL DEFAULT (now() at time zone 'utc')
)