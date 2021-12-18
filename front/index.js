import * as uuid from 'uuid';
import * as wasm from "../wasm/pkg";
import * as webgpu_server from "./webgpu_server";
import * as webgpu_renderer from "./webgpu_renderer";
import * as canvas_renderer from "./canvas_renderer";
import {
  last,
  len,
} from "./util";
function player_id() {
  return uuid.v4()
}
// const x = {
//   'server': 'browser'
// }
const x = {
  server: {
    host: '127.0.0.1',
    port: 8000,
  }
}
if (x.server === 'local') {
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
  for (var i = 0; i < 2; i++) {
    webgpu_server.push({
      command: 'add_particle',
      kind: 'WATER',
      x: Math.random(),
      y: Math.random(),
      dx: Math.random()*0.001 - 0.0005,
      dy: Math.random()*0.001 - 0.0005
    })
  }
  for (var i = 0; i < 10; i++) {
    webgpu_server.push({
      command: 'add_ship',
      plan: plan
    })
  }
} else {
  let socket = new WebSocket("ws://127.0.0.1:8000/ws");
  let data = {};
  socket.onopen = function(e) {
    console.log("[open] Connection established");
    console.log("Sending to server");
    socket.send(JSON.stringify({
      'request': 'create_sender',
      'uuid': player_id()
    }));
  };
  socket.onmessage = function(event) {
    data = event.data;
  };
  socket.onclose = function(event) {
    if (event.wasClean) {
      console.error(`[close] Connection closed cleanly, code=${event.code} reason=${event.reason}`);
    } else {
      console.error('[close] Connection died');
    }
  };
  socket.onerror = function(error) {
    console.error(`[error] ${error.message}`);
  };
  canvas_renderer.render({
    pull: function() {
      return data;
    },
    fps_counter_length: 100,
    fps_counter: [],
    image_width: 512,
    image_height: 512,
  })
}
