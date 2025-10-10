#!/bin/sh
# $HOME/github.com/loicbourgois/gravitle/lint.sh
python3 -m pylint $HOME/github.com/loicbourgois/gravitle/ai/train.py \
    --rcfile $HOME/github.com/loicbourgois/gravitle/ai/pylintrc
