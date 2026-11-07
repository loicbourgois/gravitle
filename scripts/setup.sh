#!/bin/sh
set -e
cd $HOME/github.com/loicbourgois/gravitle
python3.13 -m venv .venv
$HOME/github.com/loicbourgois/gravitle/.venv/bin/python -m \
    pip install -r $HOME/github.com/loicbourgois/gravitle/requirements.txt
