#!/usr/bin/env sh
set -o allexport; source .env; set +o allexport

cargo build --release -p $CLI_BIN --target $TARGET

rsync -arvC --progress --copy-links target/$TARGET/release/$CLI_BIN $SERVER_HOST:$SERVER_DIR/