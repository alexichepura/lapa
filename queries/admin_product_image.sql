--! create
INSERT INTO "ProductImage" (id, alt, ext, product_id)
  VALUES (:id, :alt, :ext, :product_id);

--! read_ext
SELECT
  ext  
FROM "ProductImage"
WHERE id = :id;

--! read_by_product
SELECT
  id,
  ext
FROM "ProductImage"
WHERE product_id = :product_id;

--! delete_by_id
DELETE
FROM "ProductImage"
WHERE id = :id;

--! delete_many_by_id
DELETE
FROM "ProductImage"
WHERE id = ANY(:ids);
