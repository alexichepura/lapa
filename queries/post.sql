--! post_page : (published_at?)
SELECT
    id,
    published_at,
    slug,
    title,
    description,
    text
FROM "Post"
WHERE slug = :slug AND published_at < NOW();

--! admin_post_page : (published_at?)
SELECT
    id,
    created_at,
    published_at,
    slug,
    title,
    description,
    text
FROM "Post"
WHERE id = :id;

--! admin_post_by_slug
SELECT
    id
FROM "Post"
WHERE slug = :slug;

--! post_create (published_at?) : 
INSERT INTO "Post" (published_at, title, description, text)
  VALUES (:published_at, :title, :description, :text)
  RETURNING id, created_at;

--! post_update (published_at?) :
UPDATE "Post"
SET published_at = :published_at, slug = :slug, title = :title, description = :description, text = :text
WHERE id = :id
RETURNING created_at;

--! post_images
SELECT
    id,
    alt,
    is_hero
FROM "Image"
WHERE post_id = :post_id;

--! post_list : (image_id?, alt?)
SELECT
    "Post"."id",
    "Post"."published_at",
    "Post"."slug",
    "Post"."title",
    "Post"."description",
    "Post"."text",
    "Image"."id" AS "image_id",
    "Image"."alt"
FROM "Post"
    INNER JOIN "Image" ON "Image"."post_id" = "Post"."id"
WHERE "Post"."published_at" < NOW()
AND "Image"."is_hero" = true
LIMIT 10;
