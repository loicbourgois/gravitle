#!/bin/sh
# $HOME/github.com/loicbourgois/gravitle/ai/build_training_set.sh
name=gravitle_ai_build_training_set
docker-compose \
    --file $HOME/github.com/loicbourgois/gravitle/ai/docker-compose \
    up \
    --build --force-recreate \
    --exit-code-from $name \
    $name
