#!/bin/sh
# $HOME/github.com/loicbourgois/gravitle/gravitle/build.sh
rustup default nightly
# rustup update
cargo build --release --manifest-path $HOME/github.com/loicbourgois/gravitle/gravitle/Cargo.toml