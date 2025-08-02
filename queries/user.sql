--! user_create
INSERT INTO "User" (id, username, password)
  VALUES (:id, :username, :password);

--! user_get_auth_by_username
SELECT
    id,
    password
FROM "User"
WHERE username = :username;

--! user_find_by_id
SELECT
    username
FROM "User"
WHERE id = :id;
