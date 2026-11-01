#!/bin/sh
# $HOME/github.com/loicbourgois/gravitle/front/moonlighter/build.sh
set -e
cd $HOME/github.com/loicbourgois/gravitle/front/moonlighter/engine
datetime="$(date '+%Y-%m-%d_%H-%M-%S')"
# echo "datetime: $datetime"
name="engine-$datetime"
RUSTFLAGS='--cfg getrandom_backend="wasm_js"' wasm-pack build --dev --out-name $name --target web
rm $HOME/github.com/loicbourgois/gravitle/front/moonlighter/engine-*_bg.wasm || true
source="$HOME/github.com/loicbourgois/gravitle/front/moonlighter/engine/pkg/"
dest="$HOME/github.com/loicbourgois/gravitle/front/moonlighter/"
cp $source/${name}_bg.wasm $dest/${name}_bg.wasm
cp $source/$name.js $dest/engine.js
echo "✅ Gravitle - Moonlighter - build"
