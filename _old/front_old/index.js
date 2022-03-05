import * as uuid from 'uuid';
import * as wasm from "../wasm/pkg";
import * as webgpu_server from "./webgpu_server";
import * as webgpu_renderer from "./webgpu_renderer";
import {
  last,
  len,
} from "./util";
// wasm.greet();
// function add_player(x) {
//   console.log(x.uuid)
// }
function player_id() {
  return uuid.v4()
}
webgpu_server.serve({
  grid_width: 512/2/2,
  grid_height: 512/2/2,
  interval: 5,
  cps_counter_length: 100,
})
webgpu_renderer.render({
  server: 'js-browser',
  fps_counter_length: 100,
  fps_counter: [],
  image_width: 1024/2,
  image_height: 1024/2,
  pull: webgpu_server.pull,
  player_id: player_id(),
})
const plan = [
  [0, 1, 'METAL'],
  [0, 2, 'RADAR'],
  [0, 3, 'METAL'],
  [0, 4, 'TURBO'],
  [2, 1, 'RADAR'],
  [6, 1, 'METAL'],
  [7, 1, 'TURBO'],
  [1, 0, 'METAL'],
]

for (var i = 0; i < 0; i++) {
  webgpu_server.push({
    command: 'add_particle',
    kind: 'WATER',
    x: Math.random(),
    y: Math.random(),
    dx: Math.random()*0.001 - 0.0005,
    dy: Math.random()*0.001 - 0.0005
  })
}
// for (var i = 0; i < 100; i++) {
//   webgpu_server.push({
//     command: 'add_particle',
//     kind: 'RADAR',
//     x: 0.5,
//     y: 0.5,
//     dx: 0.0,
//     dy: 0.0
//   })
// }

for (var i = 0; i < 10; i++) {
  webgpu_server.push({
    command: 'add_ship',
    plan: plan
  })
}


// add_player({
//     uuid: uuid.v4()
// })
