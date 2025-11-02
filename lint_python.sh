#!/bin/sh
set -e
cd $HOME/github.com/loicbourgois/gravitle
$HOME/github.com/loicbourgois/gravitle/.venv/bin/python -m ruff \
    --config $HOME/github.com/loicbourgois/gravitle/.ruff.toml \
    --quiet check
echo "✅ lint_python"
