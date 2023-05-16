#!/bin/sh
# $HOME/github.com/loicbourgois/gravitle/ai/prom.sh
# grafana server \
#     -homepath $HOME/grafana

$HOME/prometheus-2.44.0.darwin-amd64/prometheus \
    --config.file=$HOME/github.com/loicbourgois/gravitle/ai/prom_conf.yml
