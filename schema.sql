CREATE TABLE IF NOT EXISTS "Ssr" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "path" TEXT NOT NULL,
    "user_agent" TEXT
);
CREATE TABLE IF NOT EXISTS "User" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "username" TEXT NOT NULL,
    "password" TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS "Session" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "session" TEXT NOT NULL,
    "expires" TIMESTAMP
);
CREATE TABLE IF NOT EXISTS "Post" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "published_at" TIMESTAMP,
    "slug" TEXT NOT NULL,
    "title" TEXT NOT NULL DEFAULT '',
    "description" TEXT NOT NULL DEFAULT '',
    "text" TEXT NOT NULL DEFAULT ''
);
CREATE TABLE IF NOT EXISTS "Image" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "alt" TEXT NOT NULL,
    "ext" TEXT NOT NULL,
    "is_hero" BOOLEAN NOT NULL DEFAULT false,
    "order" INTEGER NOT NULL DEFAULT 0,
    "post_id" TEXT NOT NULL,
    CONSTRAINT "Image_post_id_fkey" FOREIGN KEY ("post_id") REFERENCES "Post" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);
CREATE TABLE IF NOT EXISTS "Settings" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "site_url" TEXT NOT NULL DEFAULT '',
    "robots_txt" TEXT NOT NULL DEFAULT '',
    "hero_width" INTEGER NOT NULL,
    "hero_height" INTEGER NOT NULL,
    "thumb_width" INTEGER NOT NULL,
    "thumb_height" INTEGER NOT NULL,
    "home_text" TEXT NOT NULL DEFAULT ''
);
CREATE UNIQUE INDEX "User_username_key" ON "User"("username");
CREATE UNIQUE INDEX "Post_slug_key" ON "Post"("slug");
