#!/bin/sh
# $HOME/github.com/loicbourgois/gravitle/front/go.sh
# cargo run --release --manifest-path $HOME/github.com/loicbourgois/gravitle/cli/Cargo.toml -- start front
docker-compose \
  --file $HOME/github.com/loicbourgois/gravitle/front/docker-compose.yml \
  up \
  --renew-anon-volumes --build --force-recreate --remove-orphans
