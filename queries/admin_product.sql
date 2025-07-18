--! create 
INSERT INTO "Product" (id, slug, meta_title, meta_description, h1, content_id)
  VALUES (:id, :slug, :meta_title, :meta_description, :h1, :content_id);

--! page : (publish_at?)
SELECT
    "Product".id,
    "Product".created_at,
    "Product".publish_at,
    "Product".slug,
    "Product".meta_title,
    "Product".meta_description,
    "Product".h1,
    "Content".id AS content_id,
    "Content".json AS content_json
FROM "Product"
    INNER JOIN "Content" ON "Content".id = "Product".content_id
WHERE "Product".id = :id;

--! update (publish_at?) :
UPDATE "Product"
SET publish_at = :publish_at, slug = :slug, meta_title = :meta_title, meta_description = :meta_description, h1 = :h1
WHERE id = :id;

-- --! delete
-- DELETE FROM "Product" WHERE id = :id;

--! list : (publish_at?, image_id?)
SELECT
    "Product".id,
    "Product".created_at,
    "Product".publish_at,
    "Product".h1,
    "ProductImage".id AS image_id
FROM "Product"
    LEFT JOIN 
        "ProductImage"
    ON 
        "Product".id = "ProductImage".product_id
        AND "ProductImage".is_hero = true;

--! by_slug
SELECT
    id
FROM "Product"
WHERE slug = :slug;

--! read_content_id
SELECT
    content_id
FROM "Product"
WHERE id = :id;

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

