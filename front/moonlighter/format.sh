#!/bin/sh
# $HOME/github.com/loicbourgois/gravitle/front/moonlighter/format.sh
set -e
cd $HOME/github.com/loicbourgois/gravitle/front/moonlighter/engine
cargo fmt
echo "✅ Gravitle - Moonlighter - format"
