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
sleep 1
echo ""
echo "http://localhost:82/moonlighter/"
echo ""
