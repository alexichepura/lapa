#!/usr/bin/env sh
# Build an app and upload it to the server.
# Usage: ./ops/build-upload.sh <web|admin|cli>
set -eu
set -a; . ./.env; set +a

app="${1:?usage: build-upload.sh <web|admin|cli>}"

case "$app" in
  web)
    pkg="$SITE_BIN"; bin="lapa_web"; unit="lapa-web"; stylance_dir="." ;;
  admin)
    pkg="$ADMIN_BIN"; bin="lapa_admin"; unit="lapa-admin"; stylance_dir="admin" ;;
  cli)
    pkg="$CLI_BIN"; bin=""; unit=""; stylance_dir="" ;;
  *)
    echo "unknown app: $app (expected web|admin|cli)" >&2
    exit 1
    ;;
esac

if [ -n "$stylance_dir" ]; then
  (cd "$stylance_dir" && stylance .)
fi

if [ "$app" = "cli" ]; then
  cargo build --release -p "$pkg" --target "$TARGET"
  rsync -v -e "ssh -p $SSH_PORT" --progress \
    "target/$TARGET/release/$pkg" "$SERVER_HOST:$SERVER_DIR/"
  exit 0
fi

LEPTOS_BIN_TARGET_TRIPLE="$TARGET" \
LEPTOS_HASH_FILES=true \
LEPTOS_HASH_FILE_NAME="hash-$pkg.txt" \
  cargo leptos build --release -p "$pkg" --features="prod"

precompress -c br,gz,de "target/$pkg/pkg"

# cargo-leptos has used both layouts across versions.
bin_path=""
for candidate in "target/$TARGET/release/$pkg" "target/server/$TARGET/release/$pkg"; do
  if [ -x "$candidate" ]; then bin_path="$candidate"; break; fi
done
if [ -z "$bin_path" ]; then
  echo "server binary for $pkg not found (looked in target/)" >&2
  exit 1
fi

hash_path=""
for candidate in "target/$TARGET/release/hash-$pkg.txt" "target/server/$TARGET/release/hash-$pkg.txt"; do
  if [ -f "$candidate" ]; then hash_path="$candidate"; break; fi
done
if [ -z "$hash_path" ]; then
  # The units run with LEPTOS_HASH_FILES=true, which panics if the file is
  # missing, so refuse to deploy a broken build.
  echo "error: hash-$pkg.txt not found under target/ (LEPTOS_HASH_FILES is on)" >&2
  exit 1
fi

rsync -arvC --exclude=.DS_Store -e "ssh -p $SSH_PORT" --progress --copy-links \
  "target/$pkg/" "$SERVER_HOST:$SERVER_DIR/$app/public/"

rsync -v -e "ssh -p $SSH_PORT" --progress \
  "$hash_path" "$SERVER_HOST:$SERVER_DIR/$app/hash.txt"

rsync -v -e "ssh -p $SSH_PORT" --progress \
  "$bin_path" "$ROOT_HOST:/usr/bin/$bin"

ssh "$ROOT_HOST" -p "$SSH_PORT" "systemctl restart $unit"
