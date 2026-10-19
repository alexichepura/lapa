-- Queries for the tower-sessions `SessionStore`. The `session` column holds the
-- JSON-encoded session data; `expires` is the record expiry; `user_id` is the
-- axum-login user id extracted on save (used to purge a user's sessions).

--! store (user_id?)
INSERT INTO "Session" (id, session, expires, user_id)
    VALUES (:id, :session, :expires, :user_id)
ON CONFLICT (id) DO UPDATE SET
    session = EXCLUDED.session,
    expires = EXCLUDED.expires,
    user_id = EXCLUDED.user_id;

--! load
SELECT session, expires FROM "Session"
WHERE id = :id AND expires > NOW();

--! delete_one_by_id
DELETE FROM "Session" WHERE id = :id;

--! delete_expired
DELETE FROM "Session" WHERE expires IS NOT NULL AND expires < NOW();

--! delete_by_username
DELETE FROM "Session"
WHERE user_id = (SELECT id FROM "User" WHERE username = :username);
