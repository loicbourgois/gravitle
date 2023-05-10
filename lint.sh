#!/bin/sh
# $HOME/github.com/loicbourgois/gravitle/lint.sh
set -e
echo "# Start"
START_TIME=$SECONDS
cd $HOME/github.com/loicbourgois/gravitle/gravithrust
rustup override set stable
echo "" > $HOME/github.com/loicbourgois/gravitle/gravithrust/src/kind_generated.rs
echo "" > $HOME/github.com/loicbourgois/gravitle/gravithrust/src/alchemy_generated.rs
cargo +nightly fmt
cargo clippy -- \
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
cargo +nightly fmt
# docker run --rm -u `id -u`:`id -g` -v $HOME/github.com/loicbourgois/gravitle:/data minlag/mermaid-cli --width 5000 --height 5000 -i alchemy.mmd -o alchemy.png -t dark -b transparent
ELAPSED_TIME=$(($SECONDS - $START_TIME))
echo "# Duration: $ELAPSED_TIME s"
