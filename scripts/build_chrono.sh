#!/bin/sh
set -e
cd $HOME/github.com/loicbourgois/gravitle/front/chrono/engine
RUSTFLAGS='--cfg getrandom_backend="wasm_js"' wasm-pack build --target web
source="$HOME/github.com/loicbourgois/gravitle/front/chrono/engine/pkg/"
dest="$HOME/github.com/loicbourgois/gravitle/front/chrono/"
cp $source/gravitle_time_trial_bg.wasm $dest/gravitle_time_trial_bg.wasm
cp $source/gravitle_time_trial.js $dest/gravitle_time_trial.js
