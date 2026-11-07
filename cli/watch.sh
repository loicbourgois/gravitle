#!/bin/zsh
# $HOME/github.com/loicbourgois/gravitle/cli/watch.sh
set -euo pipefail
cli_dir="$HOME/github.com/loicbourgois/gravitle/cli"
exec uv run --no-project python "$cli_dir/watch.py" "$@"
