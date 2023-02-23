#!/bin/sh
rustup default nightly
# rustup update
cargo build --release --manifest-path $HOME/github.com/loicbourgois/gravitle/gravitle_simd/Cargo.toml
