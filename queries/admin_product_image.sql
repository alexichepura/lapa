--! create
INSERT INTO "ProductImage" (id, alt, ext, product_id)
  VALUES (:id, :alt, :ext, :product_id);

--! read_ext
SELECT
  ext  
FROM "ProductImage"
WHERE id = :id;
