#!/usr/bin/env sh
# Apply database migrations on the server.
set -eu
set -a; . ./.env; set +a

ssh "$SERVER_HOST" -p "$SSH_PORT" "
set -e
cd '$SERVER_DIR'
RUST_LOG=info ./$CLI_BIN migrate
"
