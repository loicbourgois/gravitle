# Gravitle

## Dev
```sh
gravitle="$HOME/github.com/loicbourgois/gravitle"
alias g="cargo run --release --manifest-path $gravitle/cli/Cargo.toml -- "
g start server
g start front
```

## Deploy
```sh
g release front
g host kill && g host sync && g host run && g host log;
```
