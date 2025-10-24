#!/bin/sh
docker-compose \
  --file $HOME/github.com/loicbourgois/gravitle/docker-compose.yml \
  up \
  --renew-anon-volumes \
  --build \
  --force-recreate \
  --remove-orphans \
  generate
cp $HOME/github.com/loicbourgois/gravitle/blueprint/ship.js \
  $HOME/github.com/loicbourgois/gravitle/front/chrono/ship.js
cp $HOME/github.com/loicbourgois/gravitle/blueprint/logo.js \
  $HOME/github.com/loicbourgois/gravitle/front/logo/logo.js
