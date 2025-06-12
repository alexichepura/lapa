--! select_all_for_convert
SELECT
    id,
    ext
FROM "ProductImage";

--! delete_many_by_id
DELETE
FROM "ProductImage"
WHERE id = ANY(:ids);

--! delete_by_id
DELETE
FROM "ProductImage"
WHERE id = :id;

--! update_alt
UPDATE "ProductImage"
SET alt = :alt
WHERE id = :id;

--! update_order
UPDATE "ProductImage"
SET "order" = :order
WHERE id = :id;

--! set_hero
UPDATE "ProductImage"
SET "is_hero" = true
WHERE id = :id;

--! unset_hero
UPDATE "ProductImage"
SET "is_hero" = false
WHERE id = :id;

--! select_product_id
SELECT
    product_id 
FROM "ProductImage"
WHERE id = :id;

--! find_hero
SELECT
    id 
FROM "ProductImage"
WHERE product_id = :product_id AND is_hero = true;

--! create
INSERT INTO "ProductImage" (id, alt, ext, product_id)
  VALUES (:id, :alt, :ext, :product_id);
