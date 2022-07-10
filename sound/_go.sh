#!/bin/sh
# $HOME/github.com/loicbourgois/gravitle/sound/_go.sh
dir=$HOME/github.com/loicbourgois/gravitle/sound/
echo "Sound at http://0.0.0.0/"
dir=$dir \
  docker-compose --file $dir/docker-compose.yml up --build
