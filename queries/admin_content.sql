--! create 
INSERT INTO "Content" (id)
  VALUES (:id);

--! read
SELECT
    id,
    json
FROM "Content"
WHERE id = :id;

--! update
UPDATE "Content"
SET json = :json
WHERE id = :id;
