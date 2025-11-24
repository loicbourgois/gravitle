#!/bin/sh
set -e
$HOME/github.com/loicbourgois/gravitle/lint_python.sh
$HOME/github.com/loicbourgois/gravitle/lint_slingshot.sh
$HOME/github.com/loicbourgois/gravitle/lint_gravithrust.sh
$HOME/github.com/loicbourgois/gravitle/lint_chrono.sh
