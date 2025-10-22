# Gravitle


## Wip
```sh
$HOME/github.com/loicbourgois/gravitle/front.sh
http://localhost:82/?render=webgpu 
```


## Chrono
```sh
$HOME/github.com/loicbourgois/gravitle/generate.sh
$HOME/github.com/loicbourgois/gravitle/build_chrono.sh
$HOME/github.com/loicbourgois/gravitle/front.sh
open http://localhost:82
open http://localhost:82/?seed=efopiw-gakura&stars=1
```


## All
```sh
ls $HOME/github.com/loicbourgois/gravitle/*.sh | wc -l
cat $HOME/github.com/loicbourgois/gravitle/all.sh | grep "\.sh" | wc -l
```


## Resources
- https://nnethercote.github.io/perf-book/title-page.html


## Todo
- lint .py code
- improve render performance using webgpu
  - increase poly count for cells drawing, especially asteroids
    this should be generated
  - light up engine + draw exhaust
  - draw links
  - track ship to always center the view on it
  - handle wrap around drawing
- different ships
  - create blueprint
  - add url param for self and for ghost
- better name than "chrono"
  - light up
- intro text for chrono
  - 2 different messages
    - initial: tasked with relighting the stars 
    - welcome back captain ... 
- fuel mode for chrono
  - log fuel used
  - show at the end below duration
- star lighting up sound
- notice color
- asteroids color
- settings
  - sound
- 60 fps test
- /mining: buggy booster flares if initial browser zoom != 100%
- format .wgsl
  https://docs.rs/naga/latest/naga/
