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
INSERT INTO "Post" (id, slug, meta_title, meta_description, content_id, category_id)
  VALUES (:id, :slug, :meta_title, :meta_description, :content_id, :category_id);

--! page : (publish_at?)
SELECT
    "Post".id,
    "Post".created_at,
    "Post".publish_at,
    "Post".slug,
    "Post".meta_title,
    "Post".meta_description,
    "Post".h1,
    "Content".id AS content_id,
    "Content".json AS content_json
FROM "Post"
    INNER JOIN "Content" ON "Content".id = "Post".content_id
WHERE "Post".id = :id;

--! update (publish_at?) :
UPDATE "Post"
SET publish_at = :publish_at, slug = :slug, meta_title = :meta_title, meta_description = :meta_description
WHERE id = :id;

--! delete
DELETE FROM "Post" WHERE id = :id;

--! list : (publish_at?)
SELECT
    id,
    created_at,
    publish_at,
    meta_title
FROM "Post";
