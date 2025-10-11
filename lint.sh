#!/bin/sh
set -e
echo "# Lint - start"
START_TIME=$SECONDS
cd $HOME/github.com/loicbourgois/gravitle/gravithrust
cargo clippy --release \
    -- \
    -D warnings \
    -D clippy::pedantic \
    -A clippy::single_match \
    -A clippy::too_many_arguments \
    -A clippy::cast_precision_loss \
    -A clippy::cast_sign_loss \
    -A clippy::cast_possible_truncation \
    -A clippy::module_name_repetitions \
    -A clippy::unused_self \
    -A clippy::match_same_arms \
    -A clippy::similar_names \
    -A clippy::many_single_char_names \
    -A clippy::single_match_else \
    -A clippy::struct_field_names \
    -A clippy::useless_format \
    -A dead_code \
    -A clippy::format_push_string 
cd $HOME/github.com/loicbourgois/gravitle/front/chrono/engine
cargo clippy --release \
    -- \
    -D warnings \
    -D clippy::pedantic \
    -A clippy::cast_precision_loss \
    -A clippy::cast_possible_truncation \
    -A clippy::similar_names \
    -A clippy::module_name_repetitions \
    -A unused_variables \
    -A dead_code \
    -A unused_imports
ELAPSED_TIME=$(($SECONDS - $START_TIME))
echo "# Lint - end - $ELAPSED_TIME s"
