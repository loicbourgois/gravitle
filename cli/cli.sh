#!/bin/sh
# $HOME/github.com/loicbourgois/gravitle/cli/cli.sh
# rustup default nightly
# rustup update
cargo run --release --manifest-path $HOME/github.com/loicbourgois/gravitle/cli/Cargo.toml -- $@
