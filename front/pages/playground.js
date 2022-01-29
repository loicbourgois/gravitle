import {
  player_id,
  len
} from "../util"


let server_data = undefined;
let socket;
const Kind = {
  Metal: 2,
  Turbo: 3
}
const minimap = {};
const view = {};
const counters = {
  frame: {
    values: [],
    size: 100,
    start: undefined,
    value: undefined,
  },
  render: {
    values: [],
    size: 100,
    start: undefined,
    value: undefined,
  },
  render_minimap: {
    values: [],
    size: 100,
    start: undefined,
    value: undefined,
  },
  render_view: {
    values: [],
    size: 100,
    start: undefined,
    value: undefined,
  }
}
const urls = [
  "ws://127.0.0.1:8000/ws",
  "ws://136.243.64.165:8000/ws",
]


function playground() {
  init()
  connect()
  render()
}

function select_url_html() {
  let options = "";
  for (var url of urls) {
    options += `<option value="${url}">${url}</option>`
  }
  return `\
  <div>
    <select name="url_selector" id="url_selector">
      ${options}
    </select>
    <span id="connection_status">Connecting</span>
  </div>`
}

function init() {
  start_counter('frame');
  start_counter('render');
  document.getElementById("content").innerHTML = `\
<canvas id="canvas"></canvas>
<div id="panel">
<div id="menu">
  <a href="/playground">Playground</a>
  <a href="/gallery">Gallery</a>
  <a href="/gallery?webgpu=true">Gallery (WebGPU)</a>
</div>
  <canvas id="minimap"></canvas>
  ${select_url_html()}
  <div>
    Zoom: <input type="range" min="0" max="1000" value="800" id="zoom_slider">
  </div>
  <div>
    x: <input type="range" min="0" max="1000" value="100" id="x_slider">
  </div>
  <div>
    y: <input type="range" min="0" max="1000" value="100" id="y_slider">
  </div>
  <p id="p_step"></p>
  <p id="p_fps"></p>

  <p id="p_counter_frame"></p>
  <p id="p_counter_render"></p>
  <p id="p_counter_render_minimap"></p>
  <p id="p_counter_render_view"></p>

  <p id="p_render_duration"></p>
  <p id="p_counter_map"></p>
  <p id="p_counter_await_map"></p>
  <p id="p_counter_put_image"></p>
  <p id="p_cps"></p>
  <p id="p_compute_duration"></p>
  <p id="p_pids"></p>
  <pre id="p_counter_global"></pre>
  <pre id="p_counter_activity"></pre>
  <pre id="p_counter_collision"></pre>
  <pre id="p_counter_linked"></pre>
  <p id="p_energy"></p>
</div>`
  minimap.canvas = document.getElementById("minimap");
  minimap.context = minimap.canvas.getContext("2d");
  const s = 256;
  minimap.canvas.width  = s;
  minimap.canvas.height = s;
  minimap.canvas.style.width = s + "px";
  minimap.canvas.style.height = s + "px";
  view.canvas = document.getElementById("canvas");
  view.context = view.canvas.getContext("2d");
  view.canvas.width  = window.innerWidth;
  view.canvas.height = window.innerHeight;
  view.canvas.style.width = window.innerWidth + "px";
  view.canvas.style.height = window.innerHeight + "px";
  document.getElementById('url_selector').addEventListener('change', function() {
    connect()
  });
}


function render() {
  stop_counter('frame');
  start_counter('frame');
  start_counter('render');
  const camera = {
    x: (document.getElementById("x_slider").value )/1000,
    y:  (document.getElementById("y_slider").value )/1000,
    zoom: 1000 / (1000 - document.getElementById("zoom_slider").value )
  }
  if (server_data !== undefined) {
    const data = new DataView(server_data);
    const littleEndian = true;
    const step = data.getUint32(0*4, littleEndian);
    const width = data.getUint32(1*4, littleEndian);
    const height = data.getUint32(2*4, littleEndian);
    const i_start = data.getUint32(3*4, littleEndian);
    const i_size = data.getUint32(4*4, littleEndian);
    const j_start = data.getUint32(5*4, littleEndian);
    const j_size = data.getUint32(6*4, littleEndian);
    const part_count = data.getUint32(7*4, littleEndian);
    const view_max = get_view_max(view);
    const x_min = camera.x - 0.5 / camera.zoom * view.canvas.width  / view_max;
    const y_min = camera.y - 0.5 / camera.zoom * view.canvas.height / view_max;
    const x_max = x_min + 1.0    / camera.zoom * view.canvas.width  / view_max ;
    const y_max = y_min + 1.0    / camera.zoom * view.canvas.height / view_max;
    start_counter('render_minimap');
    reset_canvas(minimap.canvas, minimap.context, "#ffff0004")
    render_minimap(camera, minimap, view)
    for (let i = 0 ; i < part_count ; i += 2) {
      const pid = 10*4 + i*7*4
      const d = data.getFloat32(  pid+0*4, littleEndian)
      const m = data.getFloat32(  pid+1*4, littleEndian)
      const x = data.getFloat32(  pid+2*4, littleEndian)
      const y = data.getFloat32(  pid+3*4, littleEndian)
      const kind = data.getUint32(pid+6*4, littleEndian)
      const inside = x_min <= x && x <= x_max && y_min <= y && y <= y_max;
      render_p_minimap(x, y, d * 2.0, minimap.context, inside, minimap.canvas)
    }
    stop_counter('render_minimap');
    start_counter('render_view');
    reset_canvas(view.canvas, view.context, "#000000")
    const oi = (1.0 - 1.0 / camera.zoom) * 0.5
    const zok_x = oi * view.canvas.width / view_max;
    const zok_y = oi * view.canvas.height / view_max;
    const aa_x = (view_max - view.canvas.width) / view_max * 0.5;
    const aa_y = (view_max - view.canvas.height)/ view_max * 0.5;
    for (let i = 0 ; i < part_count ; i += 1) {
      const pid = 10*4 + i*7*4

      const d = data.getFloat32(  pid+0*4, littleEndian)
      const m = data.getFloat32(  pid+1*4, littleEndian)
      const x = data.getFloat32(  pid+2*4, littleEndian)
      const y = data.getFloat32(  pid+3*4, littleEndian)
      const kind = data.getUint32(pid+6*4, littleEndian)

      // if (i == 0) {
      //   console.log(kind)
      //   console.log(x)
      //   console.log(y)
      //   console.log(d)
      //   console.log(m)
      // }

      // for (let aa = 0 ; aa < 100 ; aa +=1) {
      //   console.log( data.getUint32(  pid+aa*4, littleEndian) )
      // }



      const inside = x_min <= x && x <= x_max && y_min <= y && y <= y_max;
      if (inside) {
        if (kind == Kind.Metal) {
          view.context.fillStyle = "#aaa"
        } else if (kind == Kind.Turbo) {
          view.context.fillStyle = "#faa"
        } else if (kind == 0) {
          view.context.fillStyle = "#FF0"
        } else {
          view.context.fillStyle = "#f0f"
        }

        view.context.beginPath();
        view.context.arc(
          (x + 0.5 - camera.x - zok_x - aa_x) * camera.zoom * view_max,
          (y + 0.5 - camera.y - zok_y - aa_y) * camera.zoom * view_max,
          d * view_max * 0.5 * camera.zoom,
          0, 2 * Math.PI);
        view.context.fill();
      }

    }
    stop_counter('render_view');
    document.getElementById(`p_step`).innerHTML = `Step: ${step}`
  }

  update_counters()
  document.getElementById(`p_fps`).innerHTML = `FPS: ${(1000.0/counters.frame.value).toFixed(0)}`
  window.requestAnimationFrame(function () {
    render()
  })
  stop_counter('render');
}


function reset_canvas(canvas, context, color) {
  context.beginPath();
  context.fillStyle = color;
  context.rect(
    0,
    0,
    canvas.width,
    canvas.height);
  context.fill();
}


function get_view_max(view) {
  return Math.max(view.canvas.width, view.canvas.height)
}


function render_minimap(camera, minimap, view) {
  const view_max = get_view_max(view);
  minimap.context.beginPath();
  minimap.context.fillStyle = "#ff000088";
  minimap.context.rect(
    camera.x * minimap.canvas.width - minimap.canvas.width*0.5/camera.zoom,
    camera.y * minimap.canvas.height - minimap.canvas.height*0.5/camera.zoom,
    minimap.canvas.width   / camera.zoom,
    minimap.canvas.height  / camera.zoom);
  minimap.context.fill();
  minimap.context.beginPath();
  minimap.context.fillStyle = "#FFFF0088";
  minimap.context.rect(
    camera.x*minimap.canvas.width - minimap.canvas.width*0.5/camera.zoom * view.canvas.width  / view_max,
    camera.y*minimap.canvas.height - minimap.canvas.height*0.5/camera.zoom * view.canvas.height / view_max,
    minimap.canvas.width   / camera.zoom * view.canvas.width  / view_max,
    minimap.canvas.height  / camera.zoom * view.canvas.height / view_max);
  minimap.context.fill();
  minimap.context.fillStyle = "#000"
}


function render_p_minimap(x, y, d, context, inside, canvas) {
  if (inside) {
    context.fillStyle = "#000"
  } else {
    context.fillStyle = "#888"
  }
  context.beginPath();
  context.arc(x*canvas.width, y*canvas.height, d*canvas.width*0.5, 0, 2 * Math.PI);
  context.fill();
  const full = false;
  if (full) {
    context.beginPath();
    context.arc((x+1.0)*canvas.width, y*canvas.height, d*canvas.width*0.5, 0, 2 * Math.PI);
    context.fill();
    context.beginPath();
    context.arc((x-1.0)*canvas.width, y*canvas.height, d*canvas.width*0.5, 0, 2 * Math.PI);
    context.fill();
    context.beginPath();
    context.arc(x*canvas.width, (y+1.0)*canvas.height, d*canvas.width*0.5, 0, 2 * Math.PI);
    context.fill();
    context.beginPath();
    context.arc(x*canvas.width, (y-1.0)*canvas.height, d*canvas.width*0.5, 0, 2 * Math.PI);
    context.fill();
  }
}


function start_counter(k) {
  counters[k].start = performance.now();
}
function stop_counter(k) {
  counters[k].values.push(performance.now() - counters[k].start)
}
function update_counters() {
  for (let k in counters) {
    if (len(counters[k].values) > 0) {
      while (len(counters[k].values) > counters[k].size) {
        counters[k].values.shift()
      }
      let v = 0.0;
      for (let v_ of counters[k].values) {
        v += v_;
      }
      counters[k].value = v / len(counters[k].values)
      document.getElementById(`p_counter_${k}`).innerHTML = `${k}: ${counters[k].value.toFixed(2)}ms`
    }
  }
}


function connect() {
  // TODO:
  // Eror on firefox
  // thread '<unnamed>' panicked at '[ error ] can not get websocket: WebSocket protocol error: httparse error: invalid token', src/websocket.rs:50:31
  document.getElementById("connection_status").innerHTML = "Connecting..."
  let url = document.getElementById("url_selector").selectedOptions[0].value;
  if (socket) {
    socket.close();
  }
  socket = new WebSocket(url);
  socket.binaryType = "arraybuffer";
  socket.onopen = function(e) {
    document.getElementById("connection_status").innerHTML = "Connected"
    console.log("[open] Connection established");
    socket.send(JSON.stringify({
      'request': 'create_sender',
      'uuid': player_id()
    }));
  };
  socket.onmessage = function(event) {
    server_data = event.data;
  };
  socket.onclose = function(event) {
    document.getElementById("connection_status").innerHTML = "Closed"
    if (event.wasClean) {
      console.error(`[close] Connection closed cleanly, code=${event.code} reason=${event.reason}`);
    } else {
      console.error('[close] Connection died');
      console.error('        Retrying in 1 sec.');
      window.setTimeout(function(){
        connect();
      }, 1000)
    }
  };
  socket.onerror = function(error) {
    document.getElementById("connection_status").innerHTML = "Error"
    console.error(error);
    console.error(`[error] ${error.message}`);
  };
  console.log("Waiting for server")
}

export {
  playground
}
