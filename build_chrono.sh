#!/bin/sh
set -e
cd $HOME/github.com/loicbourgois/gravitle/front/chrono/engine
cargo clippy --release \
    -- -Dwarnings -Dclippy::pedantic \
    -Aclippy::cast_precision_loss \
    -Aclippy::cast_sign_loss \
    -Aclippy::cast_possible_truncation \
    -Aclippy::similar_names \
    -Aclippy::cast_possible_wrap \
    -Aclippy::too_many_lines \
    -Aclippy::too_many_arguments \
    -Aclippy::module_name_repetitions \
    -Aunused_variables \
    -Adead_code \
    -Aunused_imports
echo "Build wasm"
RUSTFLAGS='--cfg getrandom_backend="wasm_js"' wasm-pack build --target web
echo "Copy wasm"
source="$HOME/github.com/loicbourgois/gravitle/front/chrono/engine/pkg/"
dest="$HOME/github.com/loicbourgois/gravitle/front/chrono/"
cp $source/gravitle_time_trial_bg.wasm $dest/gravitle_time_trial_bg.wasm
cp $source/gravitle_time_trial.js $dest/gravitle_time_trial.js
