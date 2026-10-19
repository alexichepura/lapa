#!/usr/bin/env sh
# DESTRUCTIVE: restore a dump onto the server and sync media up.
# Usage: ./ops/backup-upload.sh backups/lapa-<timestamp>.dump
set -eu
set -a; . ./.env; set +a

BACKUP_DIR="${BACKUP_DIR:-./backups}"
dump="${1:?usage: backup-upload.sh <dump-file>}"
test -f "$dump"

rsync -v -e "ssh -p $SSH_PORT" --progress \
  "$dump" "$SERVER_HOST:/tmp/lapa-restore.dump"

ssh "$SERVER_HOST" -p "$SSH_PORT" "
set -e
cd '$SERVER_DIR'
set -a; . ./.env; set +a
pg_restore --clean --if-exists --no-owner -d \"\$PG__URL\" /tmp/lapa-restore.dump
rm -f /tmp/lapa-restore.dump
"

rsync -a --progress \
  "$BACKUP_DIR/image_upload/" "$SERVER_HOST:$SERVER_DIR/image_upload/"
