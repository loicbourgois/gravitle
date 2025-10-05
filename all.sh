#!/bin/sh
set -e
$HOME/github.com/loicbourgois/gravitle/lint.sh
$HOME/github.com/loicbourgois/gravitle/build_chrono.sh
$HOME/github.com/loicbourgois/gravitle/build_mining.sh
$HOME/github.com/loicbourgois/gravitle/bench.sh
$HOME/github.com/loicbourgois/gravitle/test.sh
$HOME/github.com/loicbourgois/gravitle/dev.sh
