--! create 
INSERT INTO "ContentImage" (id, alt, ext, content_id)
  VALUES (:id, :alt, :ext, :content_id);

--! read_ext
SELECT
  ext  
FROM "ContentImage"
WHERE id = :id;
