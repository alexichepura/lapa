#!/usr/bin/env sh
set -o allexport; source .env; set +o allexport

rsync -arvC --progress $SERVER_HOST:$SERVER_DIR/site.db ./
rsync -arvC --progress $SERVER_HOST:$SERVER_DIR/upload ./
rsync -arvC --progress $SERVER_HOST:$SERVER_DIR/img ./