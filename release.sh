#!/bin/sh
set -e
$HOME/github.com/loicbourgois/gravitle/lint_all.sh
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
open "http://localhost:82/?debug=true&seed=0.6137747297407333&stars=1"
open "http://localhost:81/?seed=0.6137747297407333&stars=1"
