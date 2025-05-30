--! select_by_period
SELECT path FROM "Ssr"
WHERE created_at > :created_at;
