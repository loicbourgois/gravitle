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
import {gallery} from "./pages/gallery";
import {webgpu} from "./pages/webgpu";
import {localhost_3d} from "./pages/localhost_3d";
import {playground} from "./pages/playground";
if (window.location.pathname === "/") {
  playground()
} else if (window.location.pathname === "/home") {
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
