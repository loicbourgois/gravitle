#!/bin/sh
set -e
docker-compose \
  --file $HOME/github.com/loicbourgois/gravitle/docker-compose.yml \
  up \
  --renew-anon-volumes \
  --build \
  --force-recreate \
  --remove-orphans \
  front &
sleep 0.5
echo ""
echo "home:   http://localhost:82"
echo "random: http://localhost:82/random/"
echo "random: http://localhost:82/slingshot/"
echo ""
