#!/bin/sh
# $HOME/github.com/loicbourgois/gravitle/test.sh
set -e
echo "# Start"
START_TIME=$SECONDS
cd $HOME/github.com/loicbourgois/gravitle/gravithrust
RUST_BACKTRACE=1 cargo test -- --nocapture
ELAPSED_TIME=$(($SECONDS - $START_TIME))
echo "# Duration: $ELAPSED_TIME s"
