#!/usr/bin/env sh
set -o allexport; source .env; set +o allexport

LEPTOS_BIN_TARGET_TRIPLE=$TARGET \ 
LEPTOS_HASH_FILES=true \
LEPTOS_HASH_FILE_NAME=hash-site.txt \
cargo leptos build --release -p $SITE_BIN --features="prod"

# precompress --brotli --deflate --gzip --zstd target/site/pkg
# rsync -arvC --progress --copy-links target/site $SERVER_HOST:$SERVER_DIR/
# rsync -v --progress target/release/hash-site.txt $SERVER_HOST:$SERVER_DIR/
# rsync -rvC target/server/$TARGET/release/$SITE_BIN $SERVER_HOST:$SERVER_DIR/$SITE_BIN
