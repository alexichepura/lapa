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

--! settings_page
SELECT
    "id",
    "robots_txt",
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
INSERT INTO "Settings" (id, hero_height, hero_width, thumb_height, thumb_width)
  VALUES (:id, :hero_height, :hero_width, :thumb_height, :thumb_width);

--! settings_update
UPDATE "Settings"
SET robots_txt = :robots_txt, site_url = :site_url
WHERE id = :id; 

--! settings_update_home
UPDATE "Settings"
SET home_text = :home_text
WHERE id = :id; 

--! settings_update_images
UPDATE "Settings"
SET hero_height = :hero_height, hero_width = :hero_width, thumb_height = :thumb_height, thumb_width = :thumb_width
WHERE id = :id; 
