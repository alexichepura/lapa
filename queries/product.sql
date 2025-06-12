--- WEB
--! product_page : (publish_at?)
SELECT
    id,
    publish_at,
    slug,
    meta_title,
    meta_description
FROM "Product"
WHERE slug = :slug AND publish_at < NOW();

--! product_list : (image_id?, alt?)
SELECT
    "Product"."id",
    "Product"."publish_at",
    "Product"."slug",
    "Product"."meta_title",
    "Product"."meta_description",
    "ProductImage"."id" AS "image_id",
    "ProductImage"."alt"
FROM "Product"
    INNER JOIN "ProductImage" ON "ProductImage"."product_id" = "Product"."id"
WHERE "Product"."publish_at" < NOW()
AND "ProductImage"."is_hero" = true
LIMIT 10;

--- ADMIN
--! admin_product_page : (publish_at?)
SELECT
    id,
    created_at,
    publish_at,
    slug,
    meta_title,
    meta_description
FROM "Product"
WHERE id = :id;

--! admin_product_by_slug
SELECT
    id
FROM "Product"
WHERE slug = :slug;

--! admin_product_by_id_check
SELECT
    id
FROM "Product"
WHERE id = :id;

--! product_create (publish_at?) : 
INSERT INTO "Product" (id, publish_at, meta_title, meta_description)
  VALUES (:id, :publish_at, :meta_title, :meta_description)
  RETURNING created_at;

--! product_update (publish_at?) :
UPDATE "Product"
SET publish_at = :publish_at, slug = :slug, meta_title = :meta_title, meta_description = :meta_description
WHERE id = :id
RETURNING created_at;

--! product_delete
DELETE FROM "Product" WHERE id = :id;

--! product_images
SELECT
    id,
    alt,
    is_hero
FROM "ProductImage"
WHERE product_id = :product_id;

--! admin_product_images
SELECT
    id,
    alt,
    "order",
    is_hero
FROM "ProductImage"
WHERE product_id = :product_id
ORDER BY "order";

--! product_images_ids
SELECT
    id
FROM "ProductImage"
WHERE product_id = :product_id;

--! admin_product_list : (publish_at?, image_id?)
SELECT
    "Product"."id",
    "Product"."created_at",
    "Product"."publish_at",
    "Product"."meta_title",
    "ProductImage"."id" AS "image_id"
FROM "Product"
    INNER JOIN "ProductImage" ON "ProductImage"."product_id" = "Product"."id"
AND "ProductImage"."is_hero" = true;
