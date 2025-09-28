#!/bin/sh
set -e
echo "# Start"
START_TIME=$SECONDS
cd $HOME/github.com/loicbourgois/gravitle/gravithrust
cargo +nightly bench --features bench 2>&1 | tee $HOME/github.com/loicbourgois/gravitle/gravithrust/bench.txt
git diff --word-diff $HOME/github.com/loicbourgois/gravitle/gravithrust/bench.txt
ELAPSED_TIME=$(($SECONDS - $START_TIME))
echo "# Duration: $ELAPSED_TIME s"
