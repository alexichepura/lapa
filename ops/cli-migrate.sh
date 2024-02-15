#!/usr/bin/env sh
set -o allexport; source .env; set +o allexport

ssh $SERVER_HOST "
cd $SERVER_DIR
RUST_LOG=debug ./$CLI_BIN migrate
"
