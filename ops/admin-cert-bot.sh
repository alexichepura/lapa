#!/usr/bin/env sh
set -o allexport; source .env; set +o allexport

certbot certonly \
  --manual \
  --preferred-challenges dns \
  --work-dir ./ops/cert --logs-dir ./ops/cert --config-dir ./ops/cert \
  -d $ADMIN_DOMAIN