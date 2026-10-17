# Gravitle


## Development
```sh
$HOME/github.com/loicbourgois/gravitle/generate.sh
$HOME/github.com/loicbourgois/gravitle/build_chrono.sh
http://localhost:82/?debug=true


$HOME/github.com/loicbourgois/gravitle/lint.sh
$HOME/github.com/loicbourgois/gravitle/build_slingshot.sh
$HOME/github.com/loicbourgois/gravitle/front.sh
http://localhost:82
http://localhost:82/?render=webgpu
http://localhost:82/electricity/
http://localhost:82/tree/
http://localhost:82/slingshot/
http://localhost:82/random/
http://localhost:82/mining/
```


## Release
1. Run `$HOME/github.com/loicbourgois/gravitle/release.sh`
2. Push
3. Merge the PR via https://github.com/loicbourgois/gravitle/pulls
4. Wait for action to complete at https://github.com/loicbourgois/gravitle/actions 
5. Validate at https://gravitle.com


## Misc
```sh
ls $HOME/github.com/loicbourgois/gravitle/*.sh | wc -l
cat $HOME/github.com/loicbourgois/gravitle/release.sh | grep "\.sh" | wc -l
```


## Electricity
```sh
vs $HOME/github.com/loicbourgois/gravitle/front/electricity/setup.js
vs $HOME/github.com/loicbourgois/gravitle/front/slingshot/engine/src/world.rs
$HOME/github.com/loicbourgois/gravitle/build_slingshot.sh
http://localhost:82/electricity/
```


## Tree
```sh
$HOME/github.com/loicbourgois/gravitle/build_slingshot.sh
http://localhost:82/tree/
```


## Chrono
```sh
$HOME/github.com/loicbourgois/gravitle/generate.sh
$HOME/github.com/loicbourgois/gravitle/build_chrono.sh
$HOME/github.com/loicbourgois/gravitle/front.sh
open http://localhost:82
open http://localhost:82/?seed=efopiw-gakura&stars=1
```


## Slingshot
```sh
$HOME/github.com/loicbourgois/gravitle/build_slingshot.sh
$HOME/github.com/loicbourgois/gravitle/front.sh
vs $HOME/github.com/loicbourgois/gravitle/front/slingshot/index.js
http://localhost:82/slingshot/
```


## Webgpu
```sh
$HOME/github.com/loicbourgois/gravitle/front.sh
http://localhost:82/?render=webgpu 
```


## Todo
- bug: share link: after actiavedt once, does not reset
- variable links
- moddable core engine
  - kinds as json
- slingshot
  3 body: home, moon, neighboor
  - send from home to moon
  - send from moon to home 
  - send from home to neighboor using moon
- generator: json to txt
- generator: move from python to rust
- autopilot
- tree generator
- multiplayer leaderboard
  - local server
    - ping
    - validate user play
    - store play
  - deploy
- improve render performance using webgpu
  - increase poly count for cells drawing, especially asteroids
    this should be generated
  - light up engine + draw exhaust
  - draw links
  - track ship to always center the view on it
  - handle wrap around drawing
- asteroids as clumps of smaller cells
- different ships
  - create blueprint
  - add url param for self and for ghost
- better name than "chrono"
  - light up
- intro text for chrono
  - 2 different messages
    - initial: tasked with relighting the stars 
    - welcome back captain ... 
- star lighting up sound
- notice color
- asteroids color
- settings
  - sound
- 60 fps test
- /mining: buggy booster flares if initial browser zoom != 100%
- format .wgsl
  https://docs.rs/naga/latest/naga/


## Resources
- https://nnethercote.github.io/perf-book/title-page.html
- https://github.com/snapview/tokio-tungstenite
- https://en.wikipedia.org/wiki/Gravity_assist
- simd
  - https://medium.com/@oemaxwell/rust-webassembly-performance-javascript-vs-wasm-bindgen-vs-raw-wasm-with-simd-687b1dc8127b
  - https://nickb.dev/blog/authoring-a-simd-enhanced-wasm-library-with-rust/
  - https://doc.rust-lang.org/core/arch/wasm32/index.html#simd
  - https://news.ycombinator.com/item?id=45826348
