#!/bin/sh
set -e
cd $HOME/github.com/loicbourgois/gravitle/front/chrono/engine
cargo clippy --release \
    -- \
    -D warnings \
    -D clippy::pedantic \
    -A clippy::cast_precision_loss \
    -A clippy::cast_possible_truncation \
    -A clippy::similar_names \
    -A clippy::module_name_repetitions \
    -A unused_variables \
    -A dead_code \
    -A unused_imports
echo "✅ lint_chrono"
