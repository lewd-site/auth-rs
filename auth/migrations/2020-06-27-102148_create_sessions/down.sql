ALTER TABLE users ADD COLUMN refresh_token VARCHAR(32);

UPDATE users SET (refresh_token) =
    (SELECT refresh_token FROM sessions
     WHERE sessions.user_id = users.id);

DROP TABLE sessions;
