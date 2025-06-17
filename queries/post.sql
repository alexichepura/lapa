--! page
SELECT
    "Post".id,
    "Post".created_at,
    "Post".publish_at,
    "Post".slug,
    "Post".meta_title,
    "Post".meta_description,
    "Content".id AS content_id,
    "Content".json AS content_json
FROM "Post"
    INNER JOIN "Content" ON "Content".id = "Post".content_id
WHERE "Post".slug = :slug AND "Post".publish_at < NOW();
