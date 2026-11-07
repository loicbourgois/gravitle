# Gravitle - CLI - Specifications


## Tech
The CLI itself should be built using Rust edition 2024.
Helpers can be python and/or shell scripts.
Python code should live in .py files.
Python should be invoqued via uv.


## Commands
- format:   runs cargo fmt
- front:    runs $HOME/github.com/loicbourgois/gravitle/scripts/front.sh
- moon:
    - wip:  run g todo $HOME/github.com/loicbourgois/gravitle/front/moonlighter/wip.yml
- test:     runs the rust tests, and the python tests
    - all:      default
    - python:   discovers *_test.py in cli/scripts via uv
    - rust:     runs cargo test using cli/Cargo.toml
- todo:     generates improvement proposals from a YAML specification
    - requires a specification path
    - reads models, files, and rules from the specification
    - resolves relative file references beside the specification
    - reports missing files before making API requests
    - reads the OpenRouter API key as raw text from $HOME/github.com/loicbourgois/gravitle_local/openrouter_key
    - writes <specification filename>.t.p.md beside the specification
    - writes <specification filename>.t.r.<short model name>.md for each successful response
- up:       runs $HOME/github.com/loicbourgois/gravitle/scripts/up.sh
- watch:    runs $HOME/github.com/loicbourgois/gravitle/cli/scripts/watch.sh
- wip:      runs $HOME/github.com/loicbourgois/gravitle/cli/scripts/wip.sh
- review:
    review a set of files passed via .yml
    defaults to moon for now 
    auto reach full file, eg `g review 01` -> `g review $HOME/github.com/loicbourgois/gravitle/front/moonlighter/review/01_performance.yml`
    similar to todo in how it uses openrouter

    `01`should be translated to a full path, eg: $HOME/github.com/loicbourgois/gravitle/front/moonlighter/review/01_performance.yml
    no a relative path


## Todo command
For the todo, we can leverage one or more models.
As long as at least 1 model finishes, we are OK.


## Installation
The Gravitle development CLI should be made available via the `g` command.


## Auto complete
`g fr<tab>` should autocomplete to `g front`.
`g fo<tab>` should autocomplete to `g format`.
zsh should be supported as the default


## Auto rebuild
Any change to gravitle/cli should trigger a rebuild.
A succesfull rebuild should trigger an install, including auto complete.
The watch should be started via $HOME/github.com/loicbourgois/gravitle/cli/scripts/watch.sh
The watch should run in the background.
We should have an easy to check wether or not watch is active.

cli/logs/watch.log should not contain any warning

Generated files must not trigger rebuilds:
- Git internals and Python bytecode caches
- cli/target and cli/logs
- Todo prompts: *.t.p.md
- Todo responses: *.t.r.<short model name>.md


## Logs
Watch logs should live in $HOME/github.com/loicbourgois/gravitle/cli/logs/watch.log
We want to keep the last 100 lines only. Most recent at the bottom.
Each log must be timestamped with a UTC date time.

CLI logs go to $HOME/github.com/loicbourgois/gravitle/cli/logs/cli.log
Same, 100 lines only.

All CLI commands should forward their logs to files in addition to the terminal.

g front logs should live in logs/front.log, not logs/terminal.log


## Readme
All instructions in the readme should be runnable from anywhere on the machine.
It means using one of these two options:
- CLI, eg `g watch`
- full path script, eg `$HOME/github.com/loicbourgois/gravitle/cli/scripts/watch.sh`

Keep the readme barebones. We only want the essential.


## Documentation
commands.yml should contain the list of all g commands, with a short explanation


## Paths
In logs, all paths should be formatted starting with a `~`, eg ~/github.com/loicbourgois/gravitle/README.md.


## Tests
Tests should live next to the file they are testing, eg `main.rs` + `main_test.rs`.
Only test the essential.
