#!/usr/bin/env sh
# Install the systemd units and restart the services.
set -eu
set -a; . ./.env; set +a

rsync -v -e "ssh -p $SSH_PORT" --progress \
  ops/lapa-web.service "$ROOT_HOST:/etc/systemd/system/lapa-web.service"
rsync -v -e "ssh -p $SSH_PORT" --progress \
  ops/lapa-admin.service "$ROOT_HOST:/etc/systemd/system/lapa-admin.service"

ssh "$ROOT_HOST" -p "$SSH_PORT" "
set -e
systemctl daemon-reload
systemctl restart lapa-web lapa-admin
systemctl --no-pager --full status lapa-web lapa-admin | head -n 40 || true
"
