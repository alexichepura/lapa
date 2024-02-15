#!/usr/bin/env sh
set -o allexport; source .env; set +o allexport

ssh $SERVER_HOST "
cd $SERVER_DIR
lsof -t -i:4141 | xargs -r kill
RUST_LOG="info,site=debug" \
DATABASE_URL=file:$SERVER_DIR/site.db \
LEPTOS_SITE_ADDR=127.0.0.1:4141 \
LEPTOS_SITE_ROOT=$SERVER_DIR/site \
LEPTOS_OUTPUT_NAME=$SITE_BIN \
nohup $SERVER_DIR/$SITE_BIN >$SERVER_DIR/nohup.out 2>$SERVER_DIR/nohup.err &
"
