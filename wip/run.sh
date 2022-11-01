#!/bin/sh
# $HOME/github.com/loicbourgois/gravitle/wip/run.sh
rustup default nightly
# rustup update
cargo run --release --manifest-path $HOME/github.com/loicbourgois/gravitle/wip/Cargo.toml
