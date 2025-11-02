#!/bin/sh
set -e
# docker-compose \
#   --file $HOME/github.com/loicbourgois/gravitle/docker-compose.yml \
#   up \
#   --renew-anon-volumes \
#   --build \
#   --force-recreate \
#   --remove-orphans \
#   generate
cd $HOME/github.com/loicbourgois/gravitle
$HOME/github.com/loicbourgois/gravitle/.venv/bin/python -m generate.main
cp $HOME/github.com/loicbourgois/gravitle/blueprint/ship.js \
  $HOME/github.com/loicbourgois/gravitle/front/chrono/ship.js
cp $HOME/github.com/loicbourgois/gravitle/blueprint/logo.js \
  $HOME/github.com/loicbourgois/gravitle/front/logo/logo.js
cp $HOME/github.com/loicbourgois/gravitle/blueprint/random.js \
  $HOME/github.com/loicbourgois/gravitle/front/random/random.js
echo "✅ generate"
