#!/bin/sh
# $HOME/github.com/loicbourgois/gravitle/front/build.sh
set -e
echo "# Start"
START_TIME=$SECONDS
cd $HOME/github.com/loicbourgois/gravitle/gravithrust
cargo +nightly fmt
cargo test
cargo clippy -- \
    -A clippy::single_match \
    -A clippy::too_many_arguments \
    -W clippy::pedantic \
    -A clippy::cast_precision_loss \
    -A clippy::cast_sign_loss \
    -A clippy::cast_possible_truncation \
    -A clippy::module_name_repetitions \
    -A clippy::unused_self \
    -A clippy::similar_names
wasm-pack build --no-typescript --release --target web
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
ELAPSED_TIME=$(($SECONDS - $START_TIME))
echo "# Duration: $ELAPSED_TIME s"
