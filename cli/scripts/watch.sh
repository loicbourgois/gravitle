#!/bin/zsh
set -euo pipefail
script_dir="${0:A:h}"
exec uv run --no-project python "$script_dir/watch.py" "$@"
