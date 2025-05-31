--! delete_by_expiry
DELETE
FROM "Session"
WHERE (expires IS NULL OR expires < NOW())
RETURNING id;

--! count
SELECT COUNT(*)
FROM "Session";

--! store
INSERT INTO "Session" (id, session, expires)
    VALUES (:id, :session, :expires)
ON CONFLICT(id) DO UPDATE SET
    expires = EXCLUDED.expires,
    session = EXCLUDED.session;

--! load
SELECT session FROM "Session"
WHERE id = :id AND (expires IS NULL OR expires > NOW());

--! delete_one_by_id
DELETE FROM "Session" WHERE id = :id;

--! exists
SELECT COUNT(*) FROM "Session"
WHERE id = :id AND (expires IS NULL OR expires > NOW());

--! delete_all
TRUNCATE "Session";

--! get_ids
SELECT id FROM "Session"
WHERE (expires IS NULL OR expires > NOW());
