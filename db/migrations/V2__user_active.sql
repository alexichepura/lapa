-- Add an `active` flag to users so accounts can be disabled (soft-revoked)
-- without deleting the row and losing history.
--
-- `NOT NULL DEFAULT true` keeps existing rows (and the e2e seed) active. A
-- disabled user cannot log in and existing sessions fail closed on the next
-- request, because `Authentication::load_user` returns an error for inactive
-- users (see `admin/src/server/auth.rs`).

ALTER TABLE "User" ADD COLUMN "active" BOOLEAN NOT NULL DEFAULT true;
