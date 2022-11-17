#!/bin/sh
# $HOME/github.com/loicbourgois/gravitle/gravitle/format.sh
# cargo fmt --manifest-path $HOME/github.com/loicbourgois/gravitle/gravitle/Cargo.toml
cargo clippy --manifest-path $HOME/github.com/loicbourgois/gravitle/gravitle/Cargo.toml \
    -- -Dwarnings -Dclippy::pedantic \
    -Aclippy::cast_precision_loss \
    -Aclippy::cast_sign_loss \
    -Aclippy::cast_possible_truncation \
    -Aclippy::similar_names \
    -Aclippy::module_name_repetitions
# cargo clippy --release --fix --manifest-path $HOME/github.com/loicbourgois/gravitle/gravitle/Cargo.toml -- -Dwarnings -Dclippy::pedantic
