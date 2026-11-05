#!/bin/sh
# $HOME/github.com/loicbourgois/gravitle/front/moonlighter/watch.sh
set -e
WATCH_DIR="$HOME/github.com/loicbourgois/gravitle/front/moonlighter/engine"
BUILD="$HOME/github.com/loicbourgois/gravitle/front/moonlighter/scripts/build.sh"
get_checksum() {
  find "$WATCH_DIR/src" "$WATCH_DIR/Cargo.toml" "$WATCH_DIR/Cargo.lock" \
    -type f 2>/dev/null | sort | xargs cat 2>/dev/null | md5sum
}
checksum="$(get_checksum)"
echo "👀 Gravitle - Moonlighter - watching $WATCH_DIR"
"$BUILD"
while true; do
  sleep 1
  new_checksum="$(get_checksum)"
  if [ "$new_checksum" != "$checksum" ]; then
    checksum="$new_checksum"
    "$BUILD" || echo "❌ Gravitle - Moonlighter - build failed, still watching"
    sleep 1
  fi
done
