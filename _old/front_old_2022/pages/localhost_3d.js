import * as webgpu_server from "../webgpu_server";
import * as webgpu_renderer from "../webgpu_renderer";
import * as canvas_renderer from "../canvas_renderer";
import * as three_renderer from "../three/renderer";
import {
  last,
  len,
  player_id,
} from "../util";
function localhost_3d () {
  document.getElementById("content").innerHTML = `\
  <canvas id="canvas"></canvas>
  <div id="panel">
    <canvas id="minimap"></canvas>
    <p id="p_step"></p>
    <p id="p_fps"></p>
    <p id="p_render_duration"></p>
    <p id="p_cps"></p>
    <p id="p_compute_duration"></p>
    <p id="p_pids"></p>
  </div>
`
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
  function pull() {
    return data;
  }
  three_renderer.start({
    pull: pull,
    fps_counter_length: 100,
    fps_counter: [],
    image_width: 512,
    image_height: 512,
    fov: 65,
    clipping: {
      near: 0.01,
      far: 5
    }
  })
}
export {
  localhost_3d
}
