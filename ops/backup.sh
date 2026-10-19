#!/usr/bin/env sh
# Dump Postgres + uploaded originals into the local backup dir.
# Usage: ./ops/backup.sh [--with-derived]
#
# The database dump is a custom-format archive (restore with ops/backup-upload.sh).
# image_upload/ is mirrored; image_convert/ is regenerable and only included with
# --with-derived.
set -eu
set -a; . ./.env; set +a

BACKUP_DIR="${BACKUP_DIR:-./backups}"
KEEP_DAYS="${BACKUP_KEEP_DAYS:-30}"
stamp="$(date -u +%Y%m%dT%H%M%SZ)"

mkdir -p "$BACKUP_DIR"

# The server's .env holds PG__URL. Write to a .part file so a failed dump never
# looks like a valid backup.
tmp="$BACKUP_DIR/lapa-$stamp.dump.part"
ssh "$SERVER_HOST" -p "$SSH_PORT" \
  "cd '$SERVER_DIR' && set -a && . ./.env && set +a && pg_dump -Fc \"\$PG__URL\"" \
  > "$tmp"
mv "$tmp" "$BACKUP_DIR/lapa-$stamp.dump"

rsync -a --progress \
  "$SERVER_HOST:$SERVER_DIR/image_upload/" "$BACKUP_DIR/image_upload/"

if [ "${1:-}" = "--with-derived" ]; then
  rsync -a --progress \
    "$SERVER_HOST:$SERVER_DIR/image_convert/" "$BACKUP_DIR/image_convert/"
fi

find "$BACKUP_DIR" -maxdepth 1 -name 'lapa-*.dump' -mtime +"$KEEP_DAYS" -delete

printf 'backup: %s/lapa-%s.dump\n' "$BACKUP_DIR" "$stamp"
