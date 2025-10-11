#!/bin/sh
set -e
echo "# Start"
START_TIME=$SECONDS
cd $HOME/github.com/loicbourgois/gravitle/gravithrust
echo "" > $HOME/github.com/loicbourgois/gravitle/gravithrust/src/kind_generated.rs
echo "" > $HOME/github.com/loicbourgois/gravitle/gravithrust/src/alchemy_generated.rs
echo "wasm-pack build"
RUSTFLAGS='--cfg getrandom_backend="wasm_js"' wasm-pack build --no-typescript --release --target web
rm $HOME/github.com/loicbourgois/gravitle/front/mining/blueprint/* || true
rmdir $HOME/github.com/loicbourgois/gravitle/front/mining/blueprint || true
rm $HOME/github.com/loicbourgois/gravitle/front/mining/job/* || true
rmdir $HOME/github.com/loicbourgois/gravitle/front/mining/job || true
cp -r $HOME/github.com/loicbourgois/gravitle/resources/blueprint $HOME/github.com/loicbourgois/gravitle/front/mining/blueprint
cp -r $HOME/github.com/loicbourgois/gravitle/resources/job $HOME/github.com/loicbourgois/gravitle/front/mining/job
cp -r $HOME/github.com/loicbourgois/gravitle/gravithrust/pkg/ $HOME/github.com/loicbourgois/gravitle/front/mining/gravithrust/
rm $HOME/github.com/loicbourgois/gravitle/front/mining/gravithrust/package.json
rm $HOME/github.com/loicbourgois/gravitle/front/mining/gravithrust/LICENSE
rm $HOME/github.com/loicbourgois/gravitle/front/mining/gravithrust/README.md
docker run --rm -u `id -u`:`id -g` -v $HOME/github.com/loicbourgois/gravitle/gravithrust:/data minlag/mermaid-cli --width 5000 --height 5000 -i alchemy.mmd -o alchemy.png -t dark -b black || true
docker run --rm -u `id -u`:`id -g` -v $HOME/github.com/loicbourgois/gravitle/gravithrust:/data minlag/mermaid-cli --width 5000 --height 5000 -i alchemy.mmd -o alchemy.svg -t dark -b black || true
$HOME/github.com/loicbourgois/gravitle/lint_mining.sh
ELAPSED_TIME=$(($SECONDS - $START_TIME))
echo "# Duration: $ELAPSED_TIME s"
