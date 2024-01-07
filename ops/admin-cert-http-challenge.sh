#!/usr/bin/env sh
set -a; . .env; set +a;

CHALLENGE_NAME="$1"
CHALLENGE_DATA="$2"

ssh $SERVER_HOST "
mkdir -p $SERVER_DIR/admin/.well-known/acme-challenge
echo "$CHALLENGE_DATA" > $SERVER_DIR/admin/.well-known/acme-challenge/$CHALLENGE_NAME
"