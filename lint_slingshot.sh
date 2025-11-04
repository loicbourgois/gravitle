#!/bin/sh
set -e
cd $HOME/github.com/loicbourgois/gravitle/front/slingshot/engine
cargo fmt
cargo clippy --release \
    -- \
    -D warnings \
    -D clippy::pedantic \
    -A clippy::cast_possible_truncation
