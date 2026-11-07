# Gravitle - CLI

## Installation
```sh
# install
zsh "$HOME/github.com/loicbourgois/gravitle/cli/scripts/install.sh"
# start a new session
exec zsh
```


## Watch management

Start the watcher in the background:

```sh
g watch
```

Before `g` is installed, use the script directly:

```sh
zsh "$HOME/github.com/loicbourgois/gravitle/cli/scripts/watch.sh"
```

`g watch start` is equivalent to `g watch`. Starting an already active
watcher does not create another instance.

Check whether the watcher is active:

```sh
g watch status
```

This exits with status `0` when active and `1` when inactive.

Stop the watcher and any active build/install process:

```sh
g watch stop
```

The script also accepts the same management actions:

```sh
zsh "$HOME/github.com/loicbourgois/gravitle/cli/scripts/watch.sh" status
zsh "$HOME/github.com/loicbourgois/gravitle/cli/scripts/watch.sh" stop
```

Logs are written to
`$HOME/github.com/loicbourgois/gravitle/cli/logs/watch.log`.
Only the latest 100 lines are retained, with the most recent at the
bottom. Each line begins with a UTC timestamp.

The watcher performs an initial build/install, then checks for source
changes every half second. Successful installations also regenerate
completions. Git internals, the CLI's `target` and `logs` directories,
and Python bytecode caches are excluded to avoid rebuild loops.
Failed builds leave the watcher running so another edit can trigger
a retry.


## Tests
```sh
g test
```
