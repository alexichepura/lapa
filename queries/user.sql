--! user_create
INSERT INTO "User" (username, password)
  VALUES (:username, :password);

--! user_find_by_username
SELECT
    id,
    password
FROM "User"
WHERE username = :username;
