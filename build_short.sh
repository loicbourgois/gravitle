#!/bin/sh
# $HOME/github.com/loicbourgois/gravitle/build.sh
set -e
echo "# Start"
START_TIME=$SECONDS
cd $HOME/github.com/loicbourgois/gravitle/gravithrust
rustup override set stable
echo "" > $HOME/github.com/loicbourgois/gravitle/gravithrust/src/kind_generated.rs
echo "" > $HOME/github.com/loicbourgois/gravitle/gravithrust/src/alchemy_generated.rs
cargo +nightly fmt
cargo build
rm $HOME/github.com/loicbourgois/gravitle/front/blueprint/* || true
rmdir $HOME/github.com/loicbourgois/gravitle/front/blueprint || true
rm $HOME/github.com/loicbourgois/gravitle/front/job/* || true
rmdir $HOME/github.com/loicbourgois/gravitle/front/job || true
cp -r $HOME/github.com/loicbourgois/gravitle/gravithrust/src/blueprint $HOME/github.com/loicbourgois/gravitle/front/blueprint
cp -r $HOME/github.com/loicbourgois/gravitle/gravithrust/src/job $HOME/github.com/loicbourgois/gravitle/front/job
cp -r $HOME/github.com/loicbourgois/gravitle/gravithrust/pkg/ $HOME/github.com/loicbourgois/gravitle/front/gravithrust/
rm $HOME/github.com/loicbourgois/gravitle/front/gravithrust/package.json
rm $HOME/github.com/loicbourgois/gravitle/front/gravithrust/LICENSE
rm $HOME/github.com/loicbourgois/gravitle/front/gravithrust/README.md
cargo +nightly fmt
docker run --rm -u `id -u`:`id -g` -v $HOME/github.com/loicbourgois/gravitle:/data minlag/mermaid-cli --width 5000 --height 5000 -i alchemy.mmd -o alchemy.svg -t dark -b black
RUST_BACKTRACE=1 cargo test
ELAPSED_TIME=$(($SECONDS - $START_TIME))
echo "# Duration: $ELAPSED_TIME s"
