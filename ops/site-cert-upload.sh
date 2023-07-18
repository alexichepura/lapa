#!/usr/bin/env sh
set -o allexport; source .env; set +o allexport

rsync -arvC --progress --copy-links ops/cert/live/$SITE_DOMAIN/*.pem $SERVER_HOST:$SERVER_DIR/site-cert/