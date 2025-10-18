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
$HOME/github.com/loicbourgois/gravitle/*.sh | wc -l
```


## Resources
- https://nnethercote.github.io/perf-book/title-page.html


## Todo
- improve render performance using webgpu
  - add url param to choose renderer
    default to canvas until webgpu implementation is good enough
  - increase poly count for cells drawing, especially asteroids
  - light up engine + draw exhaust
  - draw links
  - track ship to always center the view on it
  - handle wrap around drawing
- better name than "chrono"
- fuel mode for chrono
  - log fuel used
  - show at the end below duration
- star light sound
- notice color
- asteroids color
- settings
- 60 fps test
- /mining: buggy booster flares if initial browser zoom != 100%
- format .wgsl
  https://docs.rs/naga/latest/naga/
