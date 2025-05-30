--! select_all_for_convert
SELECT
    id,
    ext
FROM "Image";

--! delete_many_by_id
DELETE
FROM "Image"
WHERE id = ANY(:ids);
