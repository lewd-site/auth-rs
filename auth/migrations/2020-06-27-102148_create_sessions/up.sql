CREATE TABLE sessions (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL REFERENCES users(id),
  refresh_token VARCHAR(32) NOT NULL
);

INSERT INTO sessions (user_id, refresh_token)
SELECT id, refresh_token FROM users
WHERE refresh_token IS NOT NULL;

ALTER TABLE users DROP COLUMN refresh_token;
