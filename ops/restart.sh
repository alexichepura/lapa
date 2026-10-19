#!/usr/bin/env sh
# Restart an app service.
# Usage: ./ops/restart.sh <web|admin>
set -eu
set -a; . ./.env; set +a

app="${1:?usage: restart.sh <web|admin>}"
case "$app" in
  web|admin) ;;
  *)
    echo "unknown app: $app (expected web|admin)" >&2
    exit 1
    ;;
esac

ssh "$ROOT_HOST" -p "$SSH_PORT" "systemctl restart lapa-$app"
