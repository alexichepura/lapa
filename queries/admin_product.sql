--! page : (publish_at?)
SELECT
    "Product".id,
    "Product".created_at,
    "Product".publish_at,
    "Product".slug,
    "Product".meta_title,
    "Product".meta_description,
    "Content".id AS content_id,
    "Content".json AS content_json
FROM "Product"
    INNER JOIN "Content" ON "Content".id = "Product".content_id
WHERE "Product".id = :id;

--! by_slug
SELECT
    id
FROM "Product"
WHERE slug = :slug;

--! by_id_check
SELECT
    id
FROM "Product"
WHERE id = :id;

--! create 
INSERT INTO "Product" (id, slug, meta_title, meta_description, content_id)
  VALUES (:id, :slug, :meta_title, :meta_description, :content_id);

--! update (publish_at?) :
UPDATE "Product"
SET publish_at = :publish_at, slug = :slug, meta_title = :meta_title, meta_description = :meta_description
WHERE id = :id;

--! delete
DELETE FROM "Product" WHERE id = :id;

--! images
SELECT
    id,
    alt,
    "order",
    is_hero
FROM "ProductImage"
WHERE product_id = :product_id
ORDER BY "order";

--! images_ids
SELECT
    id
FROM "ProductImage"
WHERE product_id = :product_id;

--! list : (publish_at?, image_id?)
SELECT
    "Product"."id",
    "Product"."created_at",
    "Product"."publish_at",
    "Product"."meta_title",
    "ProductImage"."id" AS "image_id"
FROM "Product"
    INNER JOIN "ProductImage" ON "ProductImage"."product_id" = "Product"."id"
AND "ProductImage"."is_hero" = true;
