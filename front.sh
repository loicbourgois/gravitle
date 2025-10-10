#!/bin/sh
docker-compose \
  --file $HOME/github.com/loicbourgois/gravitle/docker-compose.yml \
  up \
  --renew-anon-volumes \
  --build \
  --force-recreate \
  --remove-orphans \
  front
