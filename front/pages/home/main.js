import {compute_shader_0} from "./compute_shader_0";
import {reset_shader} from "./reset_shader";
import {add_parts} from "./add_parts";
import {gpu_render} from "./render";
import {
  gpu_compute,
  buffer_size,
} from "./gpu_compute";
import {
  gpu_setup,
} from "./gpu_setup";
import {
  float_size,
  attributs_count,
  little_endian,
  map_width,
  grid_width,
  map_size,
  cell_count,
} from "./constants";


const gravity_field_density = 3;
const origin = [0.0, 0.0]
const frames = []
let data_out_buffer
let gpu = {}

let context;
let canvas;
let parts = []
let gravity_field = new Array(map_size*map_size*gravity_field_density*gravity_field_density);
for (let i = 0 ; i < map_size*map_size*gravity_field_density*gravity_field_density ; i+=1 ) {
  gravity_field[i] = [0.0,0.0]
}

const byid = (id) => {
  return document.getElementById(id)
}

const value_div = (name) => {
  return `<div><label>${name}:</label><span id="value_${name}"></span></div>`
}

const go = async () => {
  const content_div = byid("content")
  content_div.innerHTML += `
    <canvas id="canvas"></canvas>
    <canvas id="gpu_canvas"></canvas>
    <div id="panel">
      ${value_div('fps')}
      ${value_div('cps')}
    </div>
  `
  canvas = byid("canvas")
  context = canvas.getContext("2d")
  canvas.width = window.innerHeight;
  canvas.height = window.innerHeight;
  await gpu_setup(gpu)
  await gpu_compute(gpu)
  gpu_render({
    buffer: gpu.buffers.in,
    device: gpu.device,
    adapter: gpu.adapter
  })
}


export {
  go
}
