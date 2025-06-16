--! list
SELECT
    id,
    created_at,
    slug,
    name
FROM "PostCategory";

--! list_for_select
SELECT
    id,
    slug,
    name
FROM "PostCategory";

--! page
SELECT
    id,
    created_at,
    slug,
    name,
    meta_title,
    meta_description
FROM "PostCategory"
WHERE id = :id;

--! create 
INSERT INTO "PostCategory" (id, slug, name, meta_title, meta_description)
  VALUES (:id, :slug, :name, :meta_title, :meta_description)
  RETURNING created_at;

--! update
UPDATE "PostCategory"
SET slug = :slug, name = :name, meta_title = :meta_title, meta_description = :meta_description
WHERE id = :id;

--! delete
DELETE FROM "PostCategory" WHERE id = :id;

