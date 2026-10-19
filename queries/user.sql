--! user_create
INSERT INTO "User" (id, username, password)
  VALUES (:id, :username, :password);

--! user_get_auth_by_username : (deactivated_at?)
SELECT
    id,
    password,
    deactivated_at
FROM "User"
WHERE username = :username;

--! user_find_by_id : (deactivated_at?)
SELECT
    username,
    deactivated_at
FROM "User"
WHERE id = :id;

--! user_set_active
-- `deactivated_at IS NULL` is the active state. Disabling stamps the time but
-- keeps the original one (COALESCE), so re-disabling does not lose history;
-- enabling clears it.
UPDATE "User"
SET deactivated_at = CASE
        WHEN :active THEN NULL
        ELSE COALESCE(deactivated_at, now())
    END
WHERE username = :username;
