#!/bin/sh
set -e
echo "Deploy - start"
rsync -a --remove-source-files $HOME/github.com/loicbourgois/gravitle/docs/* \
    $HOME/github.com/loicbourgois/gravitle/bin/ || true
find $HOME/github.com/loicbourgois/gravitle/docs/* -type d -empty -delete || true
rsync -a --exclude-from=$HOME/github.com/loicbourgois/gravitle/front/deploy_ignore \
    $HOME/github.com/loicbourgois/gravitle/front/* \
    $HOME/github.com/loicbourgois/gravitle/docs/
cp $HOME/github.com/loicbourgois/gravitle/front/index.html \
    $HOME/github.com/loicbourgois/gravitle/docs/404.html
echo "Deploy - ready"
