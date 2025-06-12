--! by_id_check
SELECT
    id
FROM "Post"
WHERE id = :id;

--! by_slug
SELECT
    id
FROM "Post"
WHERE slug = :slug;

--! create 
INSERT INTO "Post" (id, slug, meta_title, meta_description)
  VALUES (:id, :slug, :meta_title, :meta_description);

--! page : (publish_at?)
SELECT
    id,
    created_at,
    publish_at,
    slug,
    meta_title,
    meta_description
FROM "Post"
WHERE id = :id;

--! update (publish_at?) :
UPDATE "Post"
SET publish_at = :publish_at, slug = :slug, meta_title = :meta_title, meta_description = :meta_description
WHERE id = :id;

--! delete
DELETE FROM "Post" WHERE id = :id;
