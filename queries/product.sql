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
    "Product".id,
    "Product".publish_at,
    "Product".slug,
    "Product".meta_title,
    "Product".meta_description,
    "ProductImage".id AS image_id,
    "ProductImage".alt
FROM "Product"
    INNER JOIN "ProductImage" ON "ProductImage".product_id = "Product".id
WHERE "Product".publish_at < NOW()
AND "ProductImage".is_hero = true
LIMIT 10;

--! product_images
SELECT
    id,
    alt,
    is_hero
FROM "ProductImage"
WHERE product_id = :product_id;
