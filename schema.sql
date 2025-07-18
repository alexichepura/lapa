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
    "password" TEXT NOT NULL,
    "salt" TEXT NOT NULL DEFAULT ''
);
CREATE TABLE IF NOT EXISTS "Session" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "session" TEXT NOT NULL,
    "expires" TIMESTAMP
);
CREATE TABLE IF NOT EXISTS "Settings" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "site_url" TEXT NOT NULL DEFAULT '',
    "robots_txt" TEXT NOT NULL DEFAULT '',
    "home_text" TEXT NOT NULL DEFAULT ''
);
CREATE TABLE IF NOT EXISTS "Content" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "json" TEXT NOT NULL DEFAULT ''
);
CREATE TABLE IF NOT EXISTS "ContentImage" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "alt" TEXT NOT NULL,
    "ext" TEXT NOT NULL,
    "content_id" TEXT NOT NULL,
    CONSTRAINT "ContentImage_content_id_fkey" FOREIGN KEY ("content_id") REFERENCES "Content" ("id") ON DELETE CASCADE ON UPDATE CASCADE
);
CREATE TABLE IF NOT EXISTS "Product" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "publish_at" TIMESTAMP,
    "slug" TEXT NOT NULL,
    "meta_title" TEXT NOT NULL DEFAULT '',
    "meta_description" TEXT NOT NULL DEFAULT '',
    "h1" TEXT NOT NULL DEFAULT '',
    "content_id" TEXT NOT NULL,
    CONSTRAINT "Product_content_id_fkey" FOREIGN KEY ("content_id") REFERENCES "Content" ("id") ON DELETE CASCADE ON UPDATE CASCADE
);
CREATE TABLE IF NOT EXISTS "ProductImage" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "alt" TEXT NOT NULL,
    "ext" TEXT NOT NULL,
    "is_hero" BOOLEAN NOT NULL DEFAULT false,
    "order" INTEGER NOT NULL DEFAULT 0,
    "product_id" TEXT NOT NULL,
    CONSTRAINT "Image_product_id_fkey" FOREIGN KEY ("product_id") REFERENCES "Product" ("id") ON DELETE CASCADE ON UPDATE CASCADE
);
CREATE TABLE IF NOT EXISTS "PostCategory" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "slug" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "meta_title" TEXT NOT NULL DEFAULT '',
    "meta_description" TEXT NOT NULL DEFAULT ''
);
CREATE TABLE IF NOT EXISTS "Post" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "publish_at" TIMESTAMP,
    "slug" TEXT NOT NULL,
    "meta_title" TEXT NOT NULL DEFAULT '',
    "meta_description" TEXT NOT NULL DEFAULT '',
    "h1" TEXT NOT NULL DEFAULT '',
    "category_id" TEXT NOT NULL,
    "content_id" TEXT NOT NULL,
    CONSTRAINT "Post_category_id_fkey" FOREIGN KEY ("category_id") REFERENCES "PostCategory" ("id") ON DELETE RESTRICT ON UPDATE CASCADE,
    CONSTRAINT "Post_content_id_fkey" FOREIGN KEY ("content_id") REFERENCES "Content" ("id") ON DELETE CASCADE ON UPDATE CASCADE
);
CREATE UNIQUE INDEX "User_username_key" ON "User"("username");
CREATE UNIQUE INDEX "Product_slug_key" ON "Post"("slug");
CREATE UNIQUE INDEX "Category_slug_key" ON "PostCategory"("slug");
CREATE UNIQUE INDEX "Post_content_id_key" ON "Post"("content_id");
CREATE UNIQUE INDEX "Post_category_id_slug_key" ON "Post"("category_id", "slug");
