--! page
SELECT
    "Post".id,
    "Post".created_at,
    "Post".publish_at,
    "Post".slug,
    "Post".meta_title,
    "Post".meta_description,
    "Post".h1,
    "Content".id AS content_id,
    "Content".json AS content_json
FROM "Post"
    INNER JOIN "Content" ON "Content".id = "Post".content_id
    INNER JOIN "PostCategory" ON "PostCategory".id = "Post".category_id
WHERE "PostCategory".slug = :category_slug AND "Post".slug = :slug AND "Post".publish_at < NOW();

--! list
SELECT
    "Post".id,
    "Post".publish_at,
    "Post".slug,
    "Post".h1,
    "PostCategory".slug AS category_slug
FROM "Post"
    INNER JOIN "PostCategory" ON "PostCategory".id = "Post".category_id
WHERE "Post".publish_at < NOW();
