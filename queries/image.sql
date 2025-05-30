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

--! create
INSERT INTO "Image" (alt, ext, post_id)
  VALUES (:alt, :ext, :post_id)
RETURNING id;
