#!/usr/bin/env sh
set -o allexport; source .env; set +o allexport

rsync -arvC --progress --copy-links prisma $SERVER_HOST:$SERVER_DIR/
