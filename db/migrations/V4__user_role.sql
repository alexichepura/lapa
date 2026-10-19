-- Add a single role per user (admin / editor / viewer).
--
-- Existing users default to `admin`, preserving current behaviour. The CHECK
-- keeps invalid roles out; the Rust side still parses fail-closed (unknown ->
-- viewer).

ALTER TABLE "User" ADD COLUMN "role" TEXT NOT NULL DEFAULT 'admin';

ALTER TABLE "User"
    ADD CONSTRAINT "User_role_check"
    CHECK ("role" IN ('admin', 'editor', 'viewer'));
