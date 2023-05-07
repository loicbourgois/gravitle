#!/bin/sh
# $HOME/github.com/loicbourgois/gravitle/bench.sh
set -e
echo "# Start"
START_TIME=$SECONDS
cd $HOME/github.com/loicbourgois/gravitle/gravithrust
rustup override set stable
# cargo test
cargo +nightly bench --features bench
ELAPSED_TIME=$(($SECONDS - $START_TIME))
echo "# Duration: $ELAPSED_TIME s"
