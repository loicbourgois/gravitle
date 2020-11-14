'use strict';
const conf = {
  'particles': {
    'energy': {
      'r': 1.0,
      'g': 1.0,
      'b': 0.0
    },
    'rock': {
      'r': 0.5,
      'g': 0.3,
      'b': 0.1
    },
    'matter': {
      'r': 0.0,
      'g': 0.9,
      'b': 0.2
    },
    'organic_matter': {
      'r': 0.0,
      'g': 0.9,
      'b': 0.9
    },
    'waste': {
      'r': 0.8,
      'g': 0.8,
      'b': 0.8
    },
    'metal': {
      'r': 1.0,
      'g': 1.0,
      'b': 0.7
    },
    'thruster': {
      'r': 1.0,
      'g': 0.7,
      'b': 0.0
    },
  },
}
const data = {
  "width": 20.0,
  "height": 20.0,
  "base_diameter": 1.0,
  "cursor": {
    "x": 0.0,
    "y": 0.0,
  },
  "particles": [],
}
console.log("plop");
const canvas_1 = document.querySelector("#canvas_1")
canvas_1.width = window.innerHeight
canvas_1.height = window.innerHeight
const context_1 = canvas_1.getContext('2d')
let mousedown = false
canvas_1.onmousedown = function(e){
  mousedown = true
  const p = get_canvas_cursor_position(canvas_1, e)
  data.cursor.x = (p.x / canvas_1.width) * data.width;
  data.cursor.y = (1.0 - p.y / canvas_1.height) * data.height;
  data.particles.unshift({
    "type": "metal",
    "x": data.cursor.x,
    "y": data.cursor.y,
    "velocity_per_s": {
      "x": 0.0,
      "y": 0.0
    }
  });
}
canvas_1.onmousemove = function(e){
  //if (mousedown) {
    const p = get_canvas_cursor_position(canvas_1, e)
    data.cursor.x = (p.x / canvas_1.width) * data.width;
    data.cursor.y = (1.0 - p.y / canvas_1.height) * data.height;
    //console.log(data.cursor)
  //}
}
document.body.onmouseup = function(e){
  mousedown = false
}
const get_canvas_cursor_position = (canvas, event) => {
  const rect = canvas.getBoundingClientRect()
  const x = event.clientX - rect.left
  const y = event.clientY - rect.top
  return {
    x: x,
    y: y
  }
}

let center_x = 0.5;
let center_y = 0.5
let now_ms = Date.now();
let last_render_time_ms = 0;
const fps_list = [];
const render = () => {
  const zoom = parseFloat(document.querySelector("#slider_1").value) / 1000.0 * 9.0 + 1.0
  document.getElementById('x').innerHTML = data.cursor.x.toFixed(2);
  document.getElementById('y').innerHTML = data.cursor.y.toFixed(2);
  document.getElementById('particles_str').value = JSON.stringify(data.particles, null, 2);
  // fps
  last_render_time_ms = now_ms
  now_ms = Date.now()
  const elapsed_ms = now_ms - last_render_time_ms
  fps_list.push(1.0 / (elapsed_ms / 1000.0))
  while (fps_list.length > 10) {
    fps_list.shift()
  }
  let fps_sum = 0;
  for (let i = 0 ; i < fps_list.length ; i += 1) {
    fps_sum += fps_list[i]
  }
  const fps = fps_sum / fps_list.length;
  document.getElementById('fps').innerHTML = fps.toFixed(2);
  //
  context_1.clearRect(0, 0, canvas_1.width, canvas_1.height);
  //
  {
    const color = 'rgba(255.0, 255.0, 255.0, 0.5)';
    draw_disk(canvas_1, data.cursor.x, data.cursor.y, data.base_diameter, zoom, center_x, center_y, color, data);
  }
  //
  for (const i in data.particles) {
    const p = data.particles[i];
    const color = 'rgba(255.0, 100.0, 0.0, 0.8)';
    draw_disk(canvas_1, p.x, p.y, data.base_diameter, zoom, center_x, center_y, color, data);
  }
}

const log = (message) => {
  const textarea_logs = document.getElementById('logs')
  textarea_logs.value += message + '\n';
  textarea_logs.scrollTop = textarea_logs.scrollHeight;
}


const start_render_loop = () => {
  log(`starting rendering`)
  render_loop()
}
const render_loop = () => {
  render()
  setTimeout(render_loop, 0)
}
const draw_disk = (canvas, x, y, diameter, zoom, center_x, center_y, color, data) => {
  x = x / data.width;
  y = y / data.height;
  diameter = diameter / data.width;
  const p = get_canvas_coord(canvas, x, y, zoom, center_x, center_y)
  const radius_canvas = diameter * 0.5 * canvas.width * zoom;
  if (p.x > canvas.width + radius_canvas
    || p.x < - radius_canvas
    || p.y > canvas.height + radius_canvas
    || p.y < - radius_canvas)  {
    return
  }
  const startAngle = 0;
  const endAngle = Math.PI + (Math.PI * 360) * 0.5;
  const context = canvas.getContext('2d')
  context.beginPath();
  context.arc(p.x, p.y, radius_canvas, startAngle, endAngle);
  context.fillStyle = color;
  context.fill();
}
const get_canvas_coord = (canvas, x, y, zoom, center_x, center_y) => {
  y = 1.0 - y
  x = x * zoom
  x = x - center_x * zoom + 0.5
  y = y * zoom
  y = y - center_y * zoom + 0.5
  return {
    x: canvas.width * x,
    y: canvas.height * y
  }
}
start_render_loop();



/*

{
  "type": "metal",
  "x": 7.0,
  "y": 13.0,
  "velocity_per_s": {
    "x": 0.0,
    "y": 0.0
  }
}

*/
