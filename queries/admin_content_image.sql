--! create 
INSERT INTO "ContentImage" (id, alt, ext, content_id)
  VALUES (:id, :alt, :ext, :content_id);

--! read_ext
SELECT
  ext  
FROM "ContentImage"
WHERE id = :id;

--! read_by_content
SELECT
  id,
  ext  
FROM "ContentImage"
WHERE content_id = :content_id;

--! delete_many_by_id
DELETE
FROM "ContentImage"
WHERE id = ANY(:ids);
