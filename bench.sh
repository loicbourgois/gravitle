#!/bin/sh
# $HOME/github.com/loicbourgois/gravitle/bench.sh
set -e
echo "# Start"
START_TIME=$SECONDS
cd $HOME/github.com/loicbourgois/gravitle/gravithrust
rustup override set stable
cargo +nightly bench --features bench 2>&1 | tee $HOME/github.com/loicbourgois/gravitle/gravithrust/bench.txt
# cargo +nightly bench --features bench
# cargo +nightly bench --features bench collision_responses
cargo +nightly fmt
# git diff --word-diff $HOME/github.com/loicbourgois/gravitle/gravithrust/bench.txt
ELAPSED_TIME=$(($SECONDS - $START_TIME))
echo "# Duration: $ELAPSED_TIME s"
