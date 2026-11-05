#!/bin/sh
# $HOME/github.com/loicbourgois/gravitle/front/moonlighter/dev_inner.sh
set -e
docker-compose \
    --file $HOME/github.com/loicbourgois/gravitle/docker-compose.yml \
    up \
    --renew-anon-volumes \
    --build \
    --force-recreate \
    --remove-orphans \
    moonlighter_watch front &