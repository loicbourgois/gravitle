#!/bin/sh
# $HOME/github.com/loicbourgois/gravitle/front/build.sh
cd $HOME/github.com/loicbourgois/gravitle/gravithrust
cargo fmt
wasm-pack -vvv build --target web
cp -r $HOME/github.com/loicbourgois/gravitle/gravithrust/pkg/ $HOME/github.com/loicbourgois/gravitle/front/gravithrust/
rm $HOME/github.com/loicbourgois/gravitle/front/gravithrust/package.json
