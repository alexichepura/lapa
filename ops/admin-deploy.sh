#!/usr/bin/env sh
set -o allexport; source .env; set +o allexport

LEPTOS_BIN_TARGET_TRIPLE=$TARGET cargo leptos build --release -p $ADMIN_BIN
precompress --brotli --deflate --gzip --zstd target/admin/pkg

rsync -arvC --progress --copy-links target/admin $SERVER_HOST:$SERVER_DIR/
rsync -rvC target/server/$TARGET/release/$ADMIN_BIN $SERVER_HOST:$SERVER_DIR/$ADMIN_BIN