#!/usr/bin/env sh
set -a; . .env; set +a;

# https://eff-certbot.readthedocs.io/en/stable/using.html#manual
# /.well-known/acme-challenge/
CHALLENGE="http" # dns | http
certbot certonly \
  --manual \
  --preferred-challenges $CHALLENGE \
  --work-dir ./ops/cert --logs-dir ./ops/cert --config-dir ./ops/cert \
  -d $SITE_DOMAIN