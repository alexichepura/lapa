--! create (user_agent?)
INSERT INTO "Ssr" (id, path, user_agent)
  VALUES (:id, :path, :user_agent);

--! stats_all (limit, offset)
SELECT path, count(*) AS n
FROM "Ssr"
GROUP BY path
ORDER BY n DESC, path
LIMIT :limit OFFSET :offset;

--! stats_since (since, limit, offset)
SELECT path, count(*) AS n
FROM "Ssr"
WHERE created_at > :since
GROUP BY path
ORDER BY n DESC, path
LIMIT :limit OFFSET :offset;
