#!/usr/bin/env sh
set -o allexport; source .env; set +o allexport

ssh $SERVER_HOST "
cd $SERVER_DIR
lsof -t -i:4143 | xargs -r kill
DATABASE_URL=file:$SERVER_DIR/site.db \
LEPTOS_SITE_ADDR=127.0.0.1:4143 \
LEPTOS_SITE_ROOT=$SERVER_DIR/admin \
LEPTOS_OUTPUT_NAME=$ADMIN_BIN \
nohup $SERVER_DIR/$ADMIN_BIN >$SERVER_DIR/nohup-admin.out 2>$SERVER_DIR/nohup-admin.err &
"
