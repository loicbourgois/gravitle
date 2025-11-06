#!/bin/sh
set -e
cd $HOME/github.com/loicbourgois/gravitle/front/slingshot/engine
cargo fmt
cargo clippy --release \
    -- \
    -D warnings \
    -D clippy::pedantic \
    -A clippy::cast_possible_truncation \
    -A clippy::unsafe_derive_deserialize \
    -A clippy::many_single_char_names \
    -A clippy::unnecessary_get_then_check\
    -A clippy::cast_precision_loss \
    -A clippy::too_many_lines \
    -A clippy::missing_panics_doc \
    -A clippy::must_use_candidate
echo "✅ lint_slingshot"
