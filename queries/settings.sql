--! settings
SELECT
    "id",
    "home_text",
    "site_url",
    "hero_height",
    "hero_width",
    "thumb_height",
    "thumb_width"
FROM "Settings";

--! settings_robots
SELECT
    "robots_txt"
FROM "Settings";

--! settings_home
SELECT
    "home_text"
FROM "Settings";

--! settings_create
INSERT INTO "Settings" (hero_height, hero_width, thumb_height, thumb_width)
  VALUES (:hero_height, :hero_width, :thumb_height, :thumb_width);

--! settings_update
UPDATE "Settings"
SET robots_txt = :robots_txt, site_url = :site_url
WHERE id = :id; 
