#!/bin/sh
# $HOME/github.com/loicbourgois/gravitle/gravitle/format.sh
cargo fmt --manifest-path $HOME/github.com/loicbourgois/gravitle/gravitle/Cargo.toml
cargo clippy --fix --manifest-path $HOME/github.com/loicbourgois/gravitle/gravitle/Cargo.toml
