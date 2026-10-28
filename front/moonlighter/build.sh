#!/bin/sh
# $HOME/github.com/loicbourgois/gravitle/front/moonlighter/build.sh
set -e
cd $HOME/github.com/loicbourgois/gravitle/front/moonlighter/engine

RUSTFLAGS='--cfg getrandom_backend="wasm_js"' wasm-pack build --out-name engine --target web
source="$HOME/github.com/loicbourgois/gravitle/front/moonlighter/engine/pkg/"
dest="$HOME/github.com/loicbourgois/gravitle/front/moonlighter/"
cp $source/engine_bg.wasm $dest/engine_bg.wasm
cp $source/engine.js $dest/engine.js
echo "✅ Gravitle - Moonlighter - build"
