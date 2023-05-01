#!/bin/sh
# $HOME/github.com/loicbourgois/gravitle/go.sh
docker-compose \
  --file $HOME/github.com/loicbourgois/gravitle/front/docker-compose.yml \
  up \
  --renew-anon-volumes --build --force-recreate --remove-orphans
