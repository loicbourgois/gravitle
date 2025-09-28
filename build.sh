#!/bin/sh
# $HOME/github.com/loicbourgois/gravitle/build.sh
set -e
echo "# Start"
START_TIME=$SECONDS
cd $HOME/github.com/loicbourgois/gravitle/gravithrust
echo "" > $HOME/github.com/loicbourgois/gravitle/gravithrust/src/kind_generated.rs
echo "" > $HOME/github.com/loicbourgois/gravitle/gravithrust/src/alchemy_generated.rs
cargo +nightly fmt
RUST_BACKTRACE=1 cargo test
cargo clippy --release -- \
    -A clippy::single_match \
    -A clippy::too_many_arguments \
    -W clippy::pedantic \
    -A clippy::cast_precision_loss \
    -A clippy::cast_sign_loss \
    -A clippy::cast_possible_truncation \
    -A clippy::module_name_repetitions \
    -A clippy::unused_self \
    -A clippy::match_same_arms \
    -A clippy::similar_names \
    -A clippy::many_single_char_names \
    -A clippy::match_on_vec_items \
    -A clippy::single_match_else
echo "wasm-pack build"
RUSTFLAGS='--cfg getrandom_backend="wasm_js"' wasm-pack build --no-typescript --release --target web
rm $HOME/github.com/loicbourgois/gravitle/front/mining/blueprint/* || true
rmdir $HOME/github.com/loicbourgois/gravitle/front/mining/blueprint || true
rm $HOME/github.com/loicbourgois/gravitle/front/mining/job/* || true
rmdir $HOME/github.com/loicbourgois/gravitle/front/mining/job || true
cp -r $HOME/github.com/loicbourgois/gravitle/resources/blueprint $HOME/github.com/loicbourgois/gravitle/front/mining/blueprint
cp -r $HOME/github.com/loicbourgois/gravitle/resources/job $HOME/github.com/loicbourgois/gravitle/front/mining/job
cp -r $HOME/github.com/loicbourgois/gravitle/gravithrust/pkg/ $HOME/github.com/loicbourgois/gravitle/front/mining/gravithrust/
rm $HOME/github.com/loicbourgois/gravitle/front/mining/gravithrust/package.json
rm $HOME/github.com/loicbourgois/gravitle/front/mining/gravithrust/LICENSE
rm $HOME/github.com/loicbourgois/gravitle/front/mining/gravithrust/README.md
cargo +nightly fmt
docker run --rm -u `id -u`:`id -g` -v $HOME/github.com/loicbourgois/gravitle:/data minlag/mermaid-cli --width 5000 --height 5000 -i alchemy.mmd -o alchemy.png -t dark -b black || true
docker run --rm -u `id -u`:`id -g` -v $HOME/github.com/loicbourgois/gravitle:/data minlag/mermaid-cli --width 5000 --height 5000 -i alchemy.mmd -o alchemy.svg -t dark -b black || true
ELAPSED_TIME=$(($SECONDS - $START_TIME))
echo "# Duration: $ELAPSED_TIME s"
