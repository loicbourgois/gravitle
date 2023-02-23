#!/bin/sh
# $HOME/github.com/loicbourgois/gravitle/front-2/go.sh
docker-compose \
  --file $HOME/github.com/loicbourgois/gravitle/front-2/docker-compose.yml \
  up \
  --renew-anon-volumes --build --force-recreate --remove-orphans
