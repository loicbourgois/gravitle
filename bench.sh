#!/bin/sh
set -e
echo "# Bench - start"
START_TIME=$SECONDS
cd $HOME/github.com/loicbourgois/gravitle/gravithrust
batt=$(pmset -g batt)
echo $batt
if [[ "$batt" == *"Now drawing from 'Battery Power'"* ]]; then
    echo "❌ laptop needs to be plugged in before running benchmarks"
    exit 1
fi
pmset -g batt > $HOME/github.com/loicbourgois/gravitle/gravithrust/bench_setup.txt
cargo +nightly bench --features bench 2>&1 | tee $HOME/github.com/loicbourgois/gravitle/gravithrust/bench.txt
git diff --word-diff \
    $HOME/github.com/loicbourgois/gravitle/gravithrust/bench_setup.txt \
    $HOME/github.com/loicbourgois/gravitle/gravithrust/bench.txt
ELAPSED_TIME=$(($SECONDS - $START_TIME))
echo "# Bench - end - $ELAPSED_TIME s"
