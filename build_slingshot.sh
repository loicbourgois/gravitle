#!/bin/sh
set -e
cd $HOME/github.com/loicbourgois/gravitle/front/slingshot/engine
cargo fmt
cargo test -- --nocapture
wasm-pack build --target web
source="$HOME/github.com/loicbourgois/gravitle/front/slingshot/engine/pkg/"
dest="$HOME/github.com/loicbourgois/gravitle/front/slingshot/"
cp $source/gravitle_slingshot_bg.wasm $dest/gravitle_slingshot_bg.wasm
cp $source/gravitle_slingshot.js $dest/gravitle_slingshot.js
