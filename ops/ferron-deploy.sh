#!/usr/bin/env sh
# Deploy the Ferron 3 config, validate it and reload the server.
# Run from the repo root, like the other ops scripts.
set -eu
set -a; . ./.env; set +a

rsync -v -e "ssh -p $SSH_PORT" --progress \
  ops/ferron.conf "$SERVER_HOST:$SERVER_DIR/ferron.conf"

ssh "$ROOT_HOST" -p "$SSH_PORT" "
set -e
mkdir -p /etc/ferron

echo 'Validating staged config...'
/usr/sbin/ferron validate -c $SERVER_DIR/ferron.conf

install -m 0644 $SERVER_DIR/ferron.conf /etc/ferron/ferron.conf

if systemctl is-active --quiet ferron; then
  systemctl reload ferron
else
  systemctl start ferron
fi
systemctl --no-pager --full status ferron | head -n 15 || true
"
