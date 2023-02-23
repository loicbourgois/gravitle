import * as wasm from "../../wasm/pkg";
import * as webgpu_server from "../webgpu_server";
import * as webgpu_renderer from "../webgpu_renderer";
import {memory} from "../../wasm/pkg/wasm_bg.wasm"
import * as render_reset from "../shaders/render_reset";
import * as render_trace from "../shaders/render_trace";
import {
  update_fps
} from "../renderer_util";
import {
  random_int
} from "../maths";
const float_size = 4
const part_args = 13
const part_size = part_args * float_size
const max_parts_sqrt = 64
const data_size = 12 * float_size;
const max_parts = max_parts_sqrt * max_parts_sqrt // 4096
const RESOLUTION = 0.5
const kind = {
  FIREFLY: 1,
  METAL: 2,
  TURBO: 3,
  DIATOM: 4,
  NEURON: 5,
  MOUTH: 6,
  CORE: 7,
  EGG: 8,
}
function gallery () {
  let webgpu = false;
  const kvs = window.location.search.replace("?", "").split("&")
  for (var kv of kvs) {
    const k = kv.split("=")[0]
    const v = kv.split("=")[1]
    if (k === "webgpu" && v === "true") {
      webgpu = true
    }
  }
  document.getElementById("content").innerHTML = `\
<canvas id="canvas"></canvas>
<div id="panel">
<div id="menu">
  <a href="/playground">Playground</a>
  <a href="/gallery">Gallery</a>
  <a href="/gallery?webgpu=true">Gallery (WebGPU)</a>
</div>
  <canvas id="minimap"></canvas>
  <div>
    Zoom: <input type="range" min="0" max="1000" value="0" id="zoom_slider">
  </div>
  <div>
    x: <input type="range" min="0" max="1000" value="500" id="x_slider">
  </div>
  <div>
    y: <input type="range" min="0" max="1000" value="500" id="y_slider">
  </div>
  <p id="p_step"></p>
  <p id="p_fps"></p>
  <p id="p_render_duration"></p>
  <p id="p_cps"></p>
  <p id="p_compute_duration"></p>
  <p id="p_pids"></p>
  <pre id="p_counter_global"></pre>
  <pre id="p_counter_activity"></pre>
  <pre id="p_counter_collision"></pre>
  <pre id="p_counter_linked"></pre>
  <p id="p_energy"></p>
</div>`
  const server = wasm.Server.new(128, 128);
  const plan = wasm.Plan.new(kind.NEURON, 0.25, kind.NEURON);
  plan.add(0, 1, kind.MOUTH); // 2
  plan.add(1, 0, kind.CORE); // 3
  let k =  kind.TURBO;
  //k = kind.METAL
  plan.add(1, 3, k); // 4
  plan.add(3, 0, k); // 5
  plan.add(1, 4, kind.NEURON); // 6
  plan.add(5, 0, kind.NEURON); // 7
  plan.add(1, 6, kind.MOUTH);
  plan.add(7, 0, kind.MOUTH);
  plan.add(6, 4, kind.NEURON);
  plan.add(5, 7, kind.NEURON);
  server.add_entity(
    plan,
    wasm.Point.new(0.0, 0.0)
  )
  for (var i = 0; i < 10; i++) {
    server.add_entity(
      plan,
      wasm.Point.new(Math.random(), Math.random())
    )
  }
  run(server)
  if (webgpu === true) {
    if ("gpu" in navigator) {
      setup_webgpu(server)
    } else {
      const m = "Gravitle works best with WebGPU.\nInstructions on how to enable at https://web.dev/gpu/#use"
      alert(m)
      console.error(m)
      render_canvas(server)
    }
  } else {
    render_canvas(server)
  }
}


async function render_webgpu(server, x, ctx) {
  const start = performance.now();
  if (x.fps_counter === undefined) {
    x.fps_counter = [];
  }
  if (x.fps_counter_length === undefined) {
    x.fps_counter_length = 100;
  }
  const cd_max = Math.max(canvas.width, canvas.height)
  const camera = {
    x: (document.getElementById("x_slider").value )/1000,
    y:  (document.getElementById("y_slider").value )/1000,
    zoom: 1000 / (1000 - document.getElementById("zoom_slider").value )
  }
  const ctx_minimap = render_minimap(camera, cd_max)
  const minimap = ctx_minimap.canvas
  const parts = new Float32Array(new Float32Array(memory.buffer, server.parts_ptr(), part_args * server.parts_count()))

  const oi = (1.0 - 1.0 / camera.zoom) * 0.5
  const zok_x = oi * canvas.width / cd_max;
  const zok_y = oi * canvas.height / cd_max;
  // const aa_x = (cd_max - canvas.width) / cd_max * 0.5;
  // const aa_y = (cd_max - canvas.height)/ cd_max * 0.5;
  const x_min = camera.x - 0.5 / camera.zoom* x.image_width  / cd_max;
  const y_min = camera.y - 0.5 / camera.zoom* x.image_height / cd_max;
  const x_max = x_min + 1.0    / camera.zoom * x.image_width  / cd_max ;
  const y_max = y_min + 1.0    / camera.zoom * x.image_height  / cd_max;
  for (let i = 0 ; i < server.parts_count() ; i++ ) {
    const x = parts[i*part_args + 0]
    const y = parts[i*part_args + 1]
    const d = parts[i*part_args + 4]
    if (x_min <= x && x <= x_max && y_min <= y && y <= y_max) {
      render_p_minimap(x, y, d, ctx_minimap, false)
    } else {
      render_p_minimap(x, y, d, ctx_minimap, true)
    }
  }
  let write_map = x.buffers.write.mapAsync(GPUMapMode.WRITE);
  let write_data_map = x.buffers.write_data.mapAsync(GPUMapMode.WRITE);
  await write_map;
  await write_data_map;
  new Float32Array(x.buffers.write.getMappedRange()).set( parts );
  const part_count = server.parts_count()
  new Float32Array(x.buffers.write_data.getMappedRange()).set( [
    camera.zoom,
    camera.x,
    camera.y,
    part_count,
    x.image_width,
    x.image_height,
    x_min,
    y_min,
    x_max,
    y_max,
    performance.now(),
    server.get_step()/1.0
  ] );


  x.buffers.write.unmap()
  x.buffers.write_data.unmap()

  {
    const command_encoder = x.device.createCommandEncoder();
    const pass_encoder = command_encoder.beginComputePass();
    pass_encoder.setPipeline(x.compute_pipelines.reset);
    pass_encoder.setBindGroup(0, x.bind_groups.reset);
    pass_encoder.dispatch(x.image_width, x.image_height);
    pass_encoder.endPass();
    const gpu_commands = command_encoder.finish();
    x.device.queue.submit([gpu_commands]);
  }

  {
    const command_encoder = x.device.createCommandEncoder();
    command_encoder.copyBufferToBuffer(x.buffers.write, 0, x.buffers.in, 0 , x.data_buffer_size);
    command_encoder.copyBufferToBuffer(x.buffers.write_data, 0, x.buffers.in_data, 0 , data_size);
    const pass_encoder = command_encoder.beginComputePass();
    pass_encoder.setPipeline(x.compute_pipelines.trace);
    pass_encoder.setBindGroup(0, x.bind_groups.trace);
    pass_encoder.dispatch(x.dispatch, x.dispatch);
    pass_encoder.endPass();
    command_encoder.copyBufferToBuffer(x.buffers.out, 0, x.buffers.read, 0, image_buffer_size(x));
    command_encoder.copyBufferToBuffer(x.buffers.out, 0, x.buffers.previous_img, 0, image_buffer_size(x));
    const gpu_commands = command_encoder.finish();
    x.device.queue.submit([gpu_commands]);
  }
  let read_map = x.buffers.read.mapAsync(GPUMapMode.READ);
  await read_map;
  ctx.putImageData(
    new ImageData(
      Uint8ClampedArray.from(new Uint32Array(x.buffers.read.getMappedRange())),
      x.image_width,
      x.image_height,
    ),
    0, 0
  );
  x.buffers.read.unmap()
  document.getElementById("p_counter_global").innerHTML =     `Global:      ${ server.get_counter_value(0).toFixed(2) }ms`
  // document.getElementById("p_counter_activity").innerHTML =   `  Activity:  ${ server.get_counter_value(2).toFixed(2) }ms`
  // document.getElementById("p_counter_collision").innerHTML =  `  Collision: ${ server.get_counter_value(1).toFixed(2) }ms`
  // document.getElementById("p_counter_linked").innerHTML =     `    Linked:  ${ server.get_counter_value(3).toFixed(2) }ms`
  // document.getElementById("p_energy").innerHTML = `Energy: ${ server.get_total_energy().toFixed(2) }`
  update_fps(x)
  window.requestAnimationFrame(function () {
    render_webgpu(server, x, ctx)
  })
  const end = performance.now();
  x.fps_counter.push({
    start: start,
    end: end,
    duration: end - start
  })
}




async function setup_webgpu(server) {
  const adapter = await navigator.gpu.requestAdapter();
  if (!adapter) {
    console.error("No gpu adapter found")
    return;
  }
  const server_data = {
    buffer_size: data_buffer_size({})
  }
  const x = {
    data_buffer_size: server_data.buffer_size,
    image_width: Math.floor(window.innerWidth*RESOLUTION),
    image_height: Math.floor(window.innerHeight*RESOLUTION),
    max_parts_sqrt: max_parts_sqrt,
  }
  x.device = await adapter.requestDevice();
  x.buffers = {
    write: x.device.createBuffer({
      size: server_data.buffer_size,
      usage: GPUBufferUsage.MAP_WRITE | GPUBufferUsage.COPY_SRC
    }),
    in: x.device.createBuffer({
      size: server_data.buffer_size,
      usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST | GPUBufferUsage.COPY_SRC
    }),
    out: x.device.createBuffer({
      size: image_buffer_size(x),
      usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST | GPUBufferUsage.COPY_SRC
    }),
    previous_img: x.device.createBuffer({
      size: image_buffer_size(x),
      usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST | GPUBufferUsage.COPY_SRC
    }),
    read: x.device.createBuffer({
      size: image_buffer_size(x),
      usage: GPUBufferUsage.MAP_READ | GPUBufferUsage.COPY_DST
    }),
    write_data: x.device.createBuffer({
      size: data_size,
      usage: GPUBufferUsage.MAP_WRITE | GPUBufferUsage.COPY_SRC
    }),
    in_data: x.device.createBuffer({
      size: data_size,
      usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST | GPUBufferUsage.COPY_SRC
    }),
  }
  x.bind_group_layouts = {
    reset: x.device.createBindGroupLayout({
      entries: [
        // In
        {
          binding: 0,
          visibility: GPUShaderStage.COMPUTE,
          buffer: {
            type: "storage"
          }
        },
        // Out
        {
          binding: 1,
          visibility: GPUShaderStage.COMPUTE,
          buffer: {
            type: "storage"
          }
        },
      ]
    }),
    trace: x.device.createBindGroupLayout({
      entries: [
        // In
        {
          binding: 0,
          visibility: GPUShaderStage.COMPUTE,
          buffer: {
            type: "storage"
          }
        },
        // Out
        {
          binding: 1,
          visibility: GPUShaderStage.COMPUTE,
          buffer: {
            type: "storage"
          }
        },
        // In data
        {
          binding: 2,
          visibility: GPUShaderStage.COMPUTE,
          buffer: {
            type: "storage"
          }
        },
      ]
    }),
  }
  x.bind_groups = {
    reset: x.device.createBindGroup({
      layout: x.bind_group_layouts.reset,
      entries: [
        {
          binding: 0,
          resource: {
            buffer: x.buffers.previous_img
          }
        },{
          binding: 1,
          resource: {
            buffer: x.buffers.out
          }
        },
      ]
    }),
    trace: x.device.createBindGroup({
      layout: x.bind_group_layouts.trace,
      entries: [
        {
          binding: 0,
          resource: {
            buffer: x.buffers.in
          }
        },{
          binding: 1,
          resource: {
            buffer: x.buffers.out
          }
        },{
          binding: 2,
          resource: {
            buffer: x.buffers.in_data
          }
        }
      ]
    }),
  }
  x.workgroup_size = 16
  x.dispatch = Math.ceil(max_parts_sqrt / x.workgroup_size);
  x.compute_pipelines = {
    reset: x.device.createComputePipeline({
      layout: x.device.createPipelineLayout({
        bindGroupLayouts: [x.bind_group_layouts.reset]
      }),
      compute: {
        module: x.device.createShaderModule({
          code: render_reset.get(x)
        }),
        entryPoint: "main"
      }
    }),
    trace: x.device.createComputePipeline({
      layout: x.device.createPipelineLayout({
        bindGroupLayouts: [x.bind_group_layouts.trace]
      }),
      compute: {
        module: x.device.createShaderModule({
          code: render_trace.get(x)
        }),
        entryPoint: "main"
      }
    }),
  };
  const canvas = document.getElementById("canvas");
  canvas.width  = x.image_width;
  canvas.height = x.image_height;
  canvas.style.width = window.innerWidth + "px";
  canvas.style.height = window.innerHeight + "px";
  const ctx = canvas.getContext("2d");
  render_webgpu(server, x, ctx)
}


function image_buffer_size(x) {
  return x.image_width * x.image_height * 4 * 4
}
function data_buffer_size(x) {
  return max_parts * part_size
}
function render_minimap(camera, cd_max) {
  const minimap = document.getElementById("minimap");
  const resolution = 256
  minimap.width  = resolution;
  minimap.height = resolution;
  const ctx_minimap = minimap.getContext("2d");
  ctx_minimap.beginPath();
  ctx_minimap.fillStyle = "#ff000088";
  ctx_minimap.rect(
    camera.x * minimap.width - minimap.width*0.5/camera.zoom,
    camera.y * minimap.height - minimap.height*0.5/camera.zoom,
    minimap.width   / camera.zoom,
    minimap.height  / camera.zoom);
  ctx_minimap.fill();
  ctx_minimap.beginPath();
  ctx_minimap.fillStyle = "#FFFF0088";
  ctx_minimap.rect(
    camera.x*minimap.width - minimap.width*0.5/camera.zoom* canvas.width  / cd_max,
    camera.y*minimap.height - minimap.height*0.5/camera.zoom* canvas.height / cd_max,
    minimap.width   / camera.zoom * canvas.width  / cd_max,
    minimap.height  / camera.zoom * canvas.height / cd_max);
  ctx_minimap.fill();
  ctx_minimap.fillStyle = "#000"
  return ctx_minimap
}
function render_p_minimap(x, y, d, ctx_minimap, light) {
  if (light) {
    ctx_minimap.fillStyle = "#888"
  } else {
    ctx_minimap.fillStyle = "#000"
  }
  ctx_minimap.beginPath();
  ctx_minimap.arc(x*minimap.width, y*minimap.height, d*minimap.width*0.5, 0, 2 * Math.PI);
  ctx_minimap.fill();
  ctx_minimap.beginPath();
  ctx_minimap.arc((x+1.0)*minimap.width, y*minimap.height, d*minimap.width*0.5, 0, 2 * Math.PI);
  ctx_minimap.fill();
  ctx_minimap.beginPath();
  ctx_minimap.arc((x-1.0)*minimap.width, y*minimap.height, d*minimap.width*0.5, 0, 2 * Math.PI);
  ctx_minimap.fill();
  ctx_minimap.beginPath();
  ctx_minimap.arc(x*minimap.width, (y+1.0)*minimap.height, d*minimap.width*0.5, 0, 2 * Math.PI);
  ctx_minimap.fill();
  ctx_minimap.beginPath();
  ctx_minimap.arc(x*minimap.width, (y-1.0)*minimap.height, d*minimap.width*0.5, 0, 2 * Math.PI);
  ctx_minimap.fill();
}
function render_canvas(server) {
  const canvas = document.getElementById("canvas");
  canvas.width  = window.innerWidth;
  canvas.height = window.innerHeight;
  canvas.style.width = window.innerWidth + "px";
  canvas.style.height = window.innerHeight + "px";
  const ctx = canvas.getContext("2d");
  const cd_max = Math.max(canvas.width, canvas.height)
  const camera = {
    x: (document.getElementById("x_slider").value )/1000,
    y:  (document.getElementById("y_slider").value )/1000,
    zoom: 1000 / (1000 - document.getElementById("zoom_slider").value )
  }
  const ctx_minimap = render_minimap(camera, cd_max)
  const minimap = ctx_minimap.canvas
  const parts = new Float32Array(memory.buffer, server.parts_ptr(), part_args * server.parts_count());
  const oi = (1.0 - 1.0 / camera.zoom) * 0.5
  const zok_x = oi * canvas.width / cd_max;
  const zok_y = oi * canvas.height / cd_max;
  const aa_x = (cd_max - canvas.width) / cd_max * 0.5;
  const aa_y = (cd_max - canvas.height)/ cd_max * 0.5;
  for (let i = 0 ; i < server.parts_count() ; i++ ) {
    const x = parts[i*part_args + 0]
    const y = parts[i*part_args + 1]
    const d = parts[i*part_args + 4]
    render_p_minimap(x, y, d, ctx_minimap)
    ctx.fillStyle = "#aaa"
    ctx.beginPath();
    ctx.arc(
      (x + 0.5 - camera.x - zok_x - aa_x) * camera.zoom * cd_max,
      (y + 0.5 - camera.y - zok_y - aa_y) * camera.zoom * cd_max,
      d * cd_max * 0.5 * camera.zoom,
      0, 2 * Math.PI);
    ctx.fill();
  }
  window.requestAnimationFrame(function () {
    render_canvas(server)
  })
}
function run(server) {
  server.tick()
  window.setTimeout(async function(){
    await run(server);
  }, 5)
}
export {
  gallery
}
