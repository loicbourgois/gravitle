#!/bin/sh
# $HOME/github.com/loicbourgois/gravitle/gravitle/format.sh
cargo fmt --manifest-path $HOME/github.com/loicbourgois/gravitle/gravitle/Cargo.toml
cargo clippy --release --fix --manifest-path $HOME/github.com/loicbourgois/gravitle/gravitle/Cargo.toml -- -D warning 
