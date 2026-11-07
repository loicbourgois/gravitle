# Gravitle - CLI - Specifications


## Tech
The CLI itself should be built using Rust edition 2024.
Helpers can be python and/or shell scripts.
Python code should live in .py files.
Python should be invoqued via uv.


## Commands
- front:    runs $HOME/github.com/loicbourgois/gravitle/scripts/front.sh
- test:     runs the rust tests, and the python tests
    - all:      default
    - python:   ..
    - rust:     ..
- up:       runs $HOME/github.com/loicbourgois/gravitle/scripts/up.sh
- watch:    runs $HOME/github.com/loicbourgois/gravitle/cli/watch.sh
- wip:      runs $HOME/github.com/loicbourgois/gravitle/cli/wip.sh


## Installation
The Gravitle development CLI should be made available via the `g` command.


## Auto complete
`g f<tab>` should autocomplete to `g front`
zsh should be supported as the default 


## Auto rebuild
Any change to gravitle/cli should trigger a rebuild.
A succesfull rebuild should trigger an install, including auto complete.
The watch should be started via $HOME/github.com/loicbourgois/gravitle/cli/watch.sh
The watch should run in the background.
We should have an easy to check wether or not watch is active.


## Logs
Watch logs should live in $HOME/github.com/loicbourgois/gravitle/cli/logs/watch.log
We want to keep the last 100 lines only. Most recent at the bottom.
Each log must be timestamped with a UTC date time.

CLI logs go to $HOME/github.com/loicbourgois/gravitle/cli/logs/cli.log
Same, 100 lines only.


## Readme
All instructions in the readme should be runnable from anywhere on the machine.
It means using one of these two options:
- CLI, eg `g watch`
- full path script, eg `$HOME/github.com/loicbourgois/gravitle/cli/watch.sh`


## Documentation
commands.yml should contain the list of all g commands, with a short explanation


## Paths
In logs, all paths should be formatted starting with a `~`, eg ~/github.com/loicbourgois/gravitle/README.md.


## Tests
Tests should live next to the file they are testing, eg `main.rs` + `main_test.rs`.
