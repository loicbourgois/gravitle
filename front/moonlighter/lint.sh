#!/bin/sh
# $HOME/github.com/loicbourgois/gravitle/front/moonlighter/lint.sh
set -e
cd $HOME/github.com/loicbourgois/gravitle/front/moonlighter/engine
cargo clippy --release \
    -- \
    -D warnings \
    -D clippy::pedantic \
    -A clippy::cast_precision_loss \
    -A clippy::cast_possible_truncation \
    -A clippy::similar_names \
    -A clippy::module_name_repetitions \
    -A unused_variables \
    -A unused_imports \
    -A clippy::unnecessary_unwrap \
    -A clippy::unnested_or_patterns \
    -A clippy::nonminimal_bool \
    -A unused_parens \
    -A clippy::match_same_arms \
    -A clippy::too_many_lines
    
echo "✅ Gravitle - Moonlighter - lint"
