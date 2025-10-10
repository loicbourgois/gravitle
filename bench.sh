#!/bin/sh
set -e
echo "# Start"
START_TIME=$SECONDS
cd $HOME/github.com/loicbourgois/gravitle/gravithrust
pmset -g batt > $HOME/github.com/loicbourgois/gravitle/gravithrust/bench_setup.txt
cargo +nightly bench --features bench 2>&1 | tee $HOME/github.com/loicbourgois/gravitle/gravithrust/bench.txt
git diff --word-diff \
    $HOME/github.com/loicbourgois/gravitle/gravithrust/bench_setup.txt \
    $HOME/github.com/loicbourgois/gravitle/gravithrust/bench.txt
ELAPSED_TIME=$(($SECONDS - $START_TIME))
echo "# Duration: $ELAPSED_TIME s"
