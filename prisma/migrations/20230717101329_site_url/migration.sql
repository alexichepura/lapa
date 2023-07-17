-- RedefineTables
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_Settings" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "created_at" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "site_url" TEXT NOT NULL DEFAULT '',
    "robots_txt" TEXT NOT NULL DEFAULT '',
    "hero_width" INTEGER NOT NULL,
    "hero_height" INTEGER NOT NULL,
    "thumb_width" INTEGER NOT NULL,
    "thumb_height" INTEGER NOT NULL,
    "home_text" TEXT NOT NULL DEFAULT ''
);
INSERT INTO "new_Settings" ("created_at", "hero_height", "hero_width", "home_text", "id", "robots_txt", "thumb_height", "thumb_width") SELECT "created_at", "hero_height", "hero_width", "home_text", "id", "robots_txt", "thumb_height", "thumb_width" FROM "Settings";
DROP TABLE "Settings";
ALTER TABLE "new_Settings" RENAME TO "Settings";
PRAGMA foreign_key_check;
PRAGMA foreign_keys=ON;
