#!/bin/sh
# $HOME/github.com/loicbourgois/gravitle/gravitle/run.sh
rustup default nightly
# rustup update
cargo run --release --manifest-path $HOME/github.com/loicbourgois/gravitle/gravitle/Cargo.toml
