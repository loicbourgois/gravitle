#!/bin/sh
set -e
echo "# Format - start"
START_TIME=$SECONDS
cd $HOME/github.com/loicbourgois/gravitle/gravithrust
cargo +nightly fmt
cd $HOME/github.com/loicbourgois/gravitle/front/chrono/engine
cargo fmt
cd $HOME/github.com/loicbourgois/gravitle/front/chrono
dprint fmt
cd $HOME/github.com/loicbourgois/gravitle
$HOME/github.com/loicbourgois/gravitle/.venv/bin/python -m ruff format
ELAPSED_TIME=$(($SECONDS - $START_TIME))
echo "# Format - end - $ELAPSED_TIME s"
