#!/bin/sh
# $HOME/github.com/loicbourgois/gravitle/front/build.sh
set -e
cd $HOME/github.com/loicbourgois/gravitle/gravithrust
cargo fmt
wasm-pack -vvv build --target web
cargo clippy -- \
    -A clippy::single_match \
    -A clippy::too_many_arguments \
    -W clippy::pedantic \
    -A clippy::must_use_candidate \
    -A clippy::cast_precision_loss \
    -A clippy::missing_panics_doc \
    -A clippy::cast_sign_loss \
    -A clippy::cast_possible_truncation \
    -A clippy::module_name_repetitions \
    -A clippy::semicolon_if_nothing_returned \
    -A clippy::too_many_lines \
    -A clippy::unused_self \
    -A clippy::similar_names

#--fix --allow-dirty
cp -r $HOME/github.com/loicbourgois/gravitle/gravithrust/pkg/ $HOME/github.com/loicbourgois/gravitle/front/gravithrust/
rm $HOME/github.com/loicbourgois/gravitle/front/gravithrust/package.json
