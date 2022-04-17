import {compute_shader_0} from "./compute_shader_0";
import {reset_shader} from "./reset_shader";
import {add_parts} from "./add_parts";
import {gpu_render} from "./render";
import {
  gpu_compute,
  buffer_size
} from "./gpu_compute";
import {
  float_size,
  attributs_count,
  little_endian,
  map_width,
  grid_width,
  map_size,
  cell_count,
} from "./constants";


const mass = 1.0
const gravity_field_density = 3;
const origin = [0.0, 0.0]
const frames = []
let data_out_buffer
let gpu = {}
const DRAW_SQUARES = false
const DRAW_CIRCLES = true
const DRAW_COLLISIONS = false
const DRAW_ORIGIN = false
const LOOP_DRAW = false

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


const gpu_setup = async () => {
  if (! ("gpu" in navigator) ) {
    const m = "Gravitle requires WebGPU.\nInstructions on how to enable at https://web.dev/gpu/#use"
    alert(m)
    console.error(m)
    return
  }
  gpu.adapter = await navigator.gpu.requestAdapter();
  if (!gpu.adapter) {
    console.error("No gpu adapter found")
    return;
  }
  gpu.device = await gpu.adapter.requestDevice();
  gpu.buffers = {
    write: gpu.device.createBuffer({
      size: buffer_size({cell_count:cell_count}),
      usage: GPUBufferUsage.MAP_WRITE | GPUBufferUsage.COPY_SRC
    }),
    reset: gpu.device.createBuffer({
      size: buffer_size({cell_count:cell_count}),
      usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST | GPUBufferUsage.COPY_SRC
    }),
    in: gpu.device.createBuffer({
      size: buffer_size({cell_count:cell_count}),
      usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST | GPUBufferUsage.COPY_SRC
    }),
    out: gpu.device.createBuffer({
      size: buffer_size({cell_count:cell_count}),
      usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST | GPUBufferUsage.COPY_SRC
    }),
    read: gpu.device.createBuffer({
      size: buffer_size({cell_count:cell_count}),
      usage: GPUBufferUsage.MAP_READ | GPUBufferUsage.COPY_DST
    })
  }
  gpu.bind_group_layouts = {
    compute_0: gpu.device.createBindGroupLayout({
      entries: [
        { // In
          binding: 0,
          visibility: GPUShaderStage.COMPUTE,
          buffer: {
            type: "storage"
          }
        }, { // Out
          binding: 1,
          visibility: GPUShaderStage.COMPUTE,
          buffer: {
            type: "storage"
          }
        },
      ]
    }),
    reset: gpu.device.createBindGroupLayout({
      entries: [
        { // Reset
          binding: 0,
          visibility: GPUShaderStage.COMPUTE,
          buffer: {
            type: "storage"
          }
        }
      ]
    })
  };
  gpu.bind_groups = {
    compute_0: gpu.device.createBindGroup({
      layout: gpu.bind_group_layouts.compute_0,
      entries: [
        {
          binding: 0,
          resource: {
            buffer: gpu.buffers.in
          }
        },{
          binding: 1,
          resource: {
            buffer: gpu.buffers.out
          }
        },
      ]
    }),
    reset: gpu.device.createBindGroup({
      layout: gpu.bind_group_layouts.reset,
      entries: [
        {
          binding: 0,
          resource: {
            buffer: gpu.buffers.reset
          }
        }
      ]
    })
  };
  gpu.workgroup_size = 1
  gpu.dispatch = Math.floor(grid_width / gpu.workgroup_size);
  gpu.compute_pipelines = {
    compute_0: gpu.device.createComputePipeline({
      layout: gpu.device.createPipelineLayout({
        bindGroupLayouts: [gpu.bind_group_layouts.compute_0]
      }),
      compute: {
        module: gpu.device.createShaderModule({
          code: compute_shader_0({
            workgroup_size: gpu.workgroup_size,
            cell_count: cell_count,
            grid_width: grid_width,
            map_width: map_width
          })
        }),
        entryPoint: "main"
      }
    }),
    reset: gpu.device.createComputePipeline({
      layout: gpu.device.createPipelineLayout({
        bindGroupLayouts: [gpu.bind_group_layouts.reset]
      }),
      compute: {
        module: gpu.device.createShaderModule({
          code: reset_shader({
            workgroup_size: gpu.workgroup_size,
            cell_count: cell_count,
            grid_width: grid_width,
            map_width: map_width
          })
        }),
        entryPoint: "main"
      }
    }),
  };
  gpu.cell_count = cell_count
  { // Reset
    const command_encoder = gpu.device.createCommandEncoder();
    const pass_encoder = command_encoder.beginComputePass();
    pass_encoder.setPipeline(gpu.compute_pipelines.reset);
    pass_encoder.setBindGroup(0, gpu.bind_groups.reset);
    pass_encoder.dispatch(gpu.dispatch, gpu.dispatch);
    pass_encoder.endPass();
    const gpu_commands = command_encoder.finish();
    gpu.device.queue.submit([gpu_commands]);
  }
  await add_parts({
    map_width: map_width,
    gpu: gpu
  })
}


const gf_id = (i,j) => {
  return j * map_size * gravity_field_density + i
}


const normalized = (v) => {
  return normalized_2(v,distance(v))
}


const normalized_2 = (v, d) => {
  return [v[0]/d,v[1]/d]
}


const distance_sqrd = (v) => {
  return v[0]*v[0] + v[1]*v[1]
}


const distance = (v) => {
  return Math.sqrt(distance_sqrd(v))
}


const dvs_mover = (value, decalage) => {
  return (value+map_size*decalage)%map_size
}


const dvs = (a,b) => {
  const dv1 = [
    b[0] - a[0],
    b[1] - a[1],
  ]
  const dv2 = [
    dvs_mover(b[0], 0.5) - dvs_mover(a[0], 0.5),
    dvs_mover(b[1], 0.5) - dvs_mover(a[1], 0.5),
  ]
  const dv3 = [
    dvs_mover(b[0], 0.25) - dvs_mover(a[0], 0.25),
    dvs_mover(b[1], 0.25) - dvs_mover(a[1], 0.25),
  ]
  return [
    dv1,
    dv2,
    dv3,
  ]
}


const min_dv = (dvs) => {
  let min_dv_ = undefined
  let d = Infinity
  for (let dv of dvs) {
    const d_new = distance_sqrd(dv)
    if (d_new < d) {
      d = d_new;
      min_dv_ = dv
    }
  }
  return min_dv_
}



export {
  go
}
