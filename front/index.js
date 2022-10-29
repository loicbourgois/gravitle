import * as webgpu_server from "./webgpu_server";
import * as webgpu_renderer from "./webgpu_renderer";
import * as canvas_renderer from "./canvas_renderer";
import * as three_renderer from "./three/renderer";
import {
  last,
  len,
  player_id,
} from "./util";
import {home} from "./pages/home";
// TODO: fix
// import {gallery} from "./pages/gallery";
// import {webgpu} from "./pages/webgpu";
import {localhost_3d} from "./pages/localhost_3d";
import {playground} from "./pages/playground";
import {local_main} from "./pages/local/main"
import {garage_main} from "./pages/garage/main"
import {sound_main} from "./pages/sound/main"
import {journey_main} from "./pages/journey/main"
import {journey_level} from "./pages/journey/level"
import {journey_garage} from "./pages/journey/garage"
import { test } from "./pages/test"
console.log(window.location.pathname)
if (window.location.pathname === "/") {
  local_main()
}
else if (window.location.pathname === "/garage") {
  garage_main()
}
else if (window.location.pathname === "/test") {
  test()
}
else if (window.location.pathname === "/journey") {
  journey_main()
}
else if (window.location.pathname === "/journey-garage") {
  journey_garage()
}
else if (window.location.pathname.includes("/journey-") ) {
  journey_level(window.location.pathname.split('journey-')[1])
}
else if (window.location.pathname === "/sound") {
  sound_main()
}
else if (window.location.pathname === "/home") {
  home()
} else if (window.location.pathname === "/playground") {
  playground()
} else if (window.location.pathname === "/gallery") {
  gallery()
} else if (window.location.pathname === "/webgpu") {
  webgpu()
} else if (window.location.pathname === "/localhost_3d") {
  localhost_3d()
} else {
  console.error("404")
}
