#!/bin/sh
set -e
cd $HOME/github.com/loicbourgois/gravitle
git commit -am "up"
git push
git status
