--! user_create
INSERT INTO "User" (username, password)
  VALUES (:username, :password);
