#!/bin/zsh
# $HOME/github.com/loicbourgois/gravitle/cli/install.sh
set -euo pipefail

cli_dir="$HOME/github.com/loicbourgois/gravitle/cli"
bin_dir="$HOME/.local/bin"
bash_completion_dir="$HOME/.local/share/bash-completion/completions"
zsh_completion_dir="$HOME/.local/share/zsh/site-functions"
zshrc="${ZDOTDIR:-$HOME}/.zshrc"
completion_marker="# Gravitle CLI completion"

mkdir -p "$bin_dir" "$bash_completion_dir" "$zsh_completion_dir"
cargo install --path "$cli_dir" --root "$HOME/.local"

"$bin_dir/g" completions bash > "$bash_completion_dir/g"
"$bin_dir/g" completions zsh > "$zsh_completion_dir/_g"

mkdir -p "$(dirname "$zshrc")"
touch "$zshrc"

if ! grep -Fqx "$completion_marker" "$zshrc"; then
    cat >> "$zshrc" <<'EOF'

# Gravitle CLI completion
fpath=("$HOME/.local/share/zsh/site-functions" $fpath)
if (( ! $+functions[compdef] )); then
    autoload -Uz compinit
    compinit
fi
autoload -Uz _g
compdef _g g
# End Gravitle CLI completion
EOF
fi

printf 'Installed g to %s\n' "$bin_dir/g"
printf 'Ensure %s is on PATH.\n' "$bin_dir"
printf 'Zsh completion configured in %s. Start a new Zsh session to enable it.\n' "$zshrc"
printf 'Bash completion installed to %s/g.\n' "$bash_completion_dir"