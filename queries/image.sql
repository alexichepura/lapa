--! select_all_for_convert
SELECT
    id,
    ext
FROM "Image";

--! delete_many_by_id
DELETE
FROM "Image"
WHERE id = ANY(:ids);

--! delete_by_id
DELETE
FROM "Image"
WHERE id = :id;

--! update_alt
UPDATE "Image"
SET alt = :alt
WHERE id = :id;

--! update_order
UPDATE "Image"
SET "order" = :order
WHERE id = :id;

--! set_hero
UPDATE "Image"
SET "is_hero" = true
WHERE id = :id;

--! unset_hero
UPDATE "Image"
SET "is_hero" = false
WHERE id = :id;

--! select_post_id
SELECT
    post_id 
FROM "Image"
WHERE id = :id;

--! find_hero
SELECT
    id 
FROM "Image"
WHERE post_id = :post_id AND is_hero = true;

--! create
INSERT INTO "Image" (alt, ext, post_id)
  VALUES (:alt, :ext, :post_id)
RETURNING id;
