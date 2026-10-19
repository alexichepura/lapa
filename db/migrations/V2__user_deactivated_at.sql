-- Allow accounts to be disabled (soft-revoked) without deleting the row and
-- losing history, and record *when* that happened.
--
-- `NULL` means the account is active; a timestamp marks when it was disabled.
-- A disabled user cannot log in and existing sessions fail closed on the next
-- request, because `Authentication::load_user` errors when `deactivated_at` is
-- set (see `admin/src/server/auth.rs`).
--
-- Nullable by design: every existing row (and the e2e seed) is active.

ALTER TABLE "User" ADD COLUMN "deactivated_at" TIMESTAMP;
