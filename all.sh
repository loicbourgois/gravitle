#!/bin/sh
set -e
$HOME/github.com/loicbourgois/gravitle/lint.sh
$HOME/github.com/loicbourgois/gravitle/build_chrono.sh
$HOME/github.com/loicbourgois/gravitle/test.sh
$HOME/github.com/loicbourgois/gravitle/bench.sh
$HOME/github.com/loicbourgois/gravitle/build_mining.sh
$HOME/github.com/loicbourgois/gravitle/generate.sh
$HOME/github.com/loicbourgois/gravitle/format.sh
$HOME/github.com/loicbourgois/gravitle/predeploy.sh
$HOME/github.com/loicbourgois/gravitle/front.sh &
$HOME/github.com/loicbourgois/gravitle/front_deployed.sh &
sleep 1
open "http://localhost:82/?seed=efopiw-gakura&stars=1"
open "http://localhost:81/?seed=efopiw-gakura&stars=1"
