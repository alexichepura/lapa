-- Reshape "Session" for tower-sessions and add a `user_id` so a user's sessions
-- can be purged on disable.
--
-- The stored session format changes (axum_session -> tower-sessions), so any
-- existing rows are unusable and are dropped; sessions are re-created on the
-- next request (one-time re-login on deploy).

DELETE FROM "Session";

ALTER TABLE "Session" ADD COLUMN "user_id" TEXT;

CREATE INDEX "Session_user_id_idx" ON "Session"("user_id");
