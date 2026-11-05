#!/bin/sh
# $HOME/github.com/loicbourgois/gravitle/front/moonlighter/build.sh
set -e
cd $HOME/github.com/loicbourgois/gravitle/front/moonlighter/engine
datetime="$(date '+%Y-%m-%d_%H-%M-%S')"
name="engine-$datetime"
RUSTFLAGS='--cfg getrandom_backend="wasm_js"' wasm-pack build --dev --out-name $name --target web
dest="$HOME/github.com/loicbourgois/gravitle/front/moonlighter/generated"
source="$HOME/github.com/loicbourgois/gravitle/front/moonlighter/engine/pkg/"
rm ${dest}/engine-*_bg.wasm || true
cp $source/${name}_bg.wasm $dest/${name}_bg.wasm
cp $source/$name.js $dest/engine.js
echo "✅ Gravitle - Moonlighter - build"
