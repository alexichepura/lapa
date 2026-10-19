--! user_create
INSERT INTO "User" (id, username, password)
  VALUES (:id, :username, :password);

--! user_get_auth_by_username
SELECT
    id,
    password,
    active
FROM "User"
WHERE username = :username;

--! user_find_by_id
SELECT
    username,
    active
FROM "User"
WHERE id = :id;

--! user_set_active
UPDATE "User"
SET active = :active
WHERE username = :username;
