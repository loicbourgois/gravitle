import {compute_shader_0} from "./compute_shader_0";
import {reset_shader} from "./reset_shader";
import {add_parts} from "./add_parts";
import {gpu_render} from "./render";
import {
  float_size,
  attributs_count,
  little_endian,
  map_width,
  grid_width,
  map_size,
  cell_count,
} from "./constants";

// const map_width = 32;
// const grid_width = map_width*2
// const map_size = map_width;
// const cell_count = grid_width * grid_width
// const little_endian = true;
const mass = 1.0
const gravity_field_density = 3;
const origin = [0.0, 0.0]
const frames = []
const computes = []
// const float_size = 4
// const attributs_count = 8
let data_out_buffer
let gpu
const DRAW_SQUARES = false
const DRAW_CIRCLES = true
const DRAW_COLLISIONS = false
const DRAW_ORIGIN = false
const ITER = 10
const LOOP_DRAW = false
const LOOP_COMPUTE = true

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
  await gpu_setup()
  await gpu_compute()
  //draw()
  gpu_render({
    buffer: gpu.buffers.in,
    device: gpu.device,
    adapter: gpu.adapter
  })
}


const buffer_size = (x) => {
  return x.cell_count * attributs_count * float_size;
}


const gpu_setup = async () => {
  gpu = {}
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
  gpu.workgroup_size = 8
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





let from_write  = true

const gpu_compute = async () => {




  for (var i = 0; i < ITER; i++)
  {
    computes.push({
      time: performance.now()
    })
    while (computes.length > 100) {
      computes.shift()
    }
    const cps_value = 1000/((computes[computes.length-1].time - computes[0].time) / computes.length)
    byid("value_cps").innerHTML = cps_value.toFixed(1)

    const command_encoder = gpu.device.createCommandEncoder();
    if (from_write) {
      command_encoder.copyBufferToBuffer(
        gpu.buffers.write, 0,
        gpu.buffers.in, 0 ,
        buffer_size({cell_count:gpu.cell_count}));
        from_write = false
    }
    const pass_encoder = command_encoder.beginComputePass();
    pass_encoder.setPipeline(gpu.compute_pipelines.compute_0);
    pass_encoder.setBindGroup(0, gpu.bind_groups.compute_0);
    pass_encoder.dispatch(gpu.dispatch, gpu.dispatch);
    pass_encoder.endPass();
    command_encoder.copyBufferToBuffer(
      gpu.buffers.out, 0,
      gpu.buffers.read, 0,
      buffer_size({cell_count:gpu.cell_count}));
    command_encoder.copyBufferToBuffer(
      gpu.buffers.out, 0,
      gpu.buffers.in, 0 ,
      buffer_size({cell_count:gpu.cell_count}));
    command_encoder.copyBufferToBuffer(
      gpu.buffers.reset, 0,
      gpu.buffers.out, 0 ,
      buffer_size({cell_count:gpu.cell_count}));
    const gpu_commands = command_encoder.finish();
    gpu.device.queue.submit([gpu_commands]);
  }



  await gpu.buffers.read.mapAsync(GPUMapMode.READ);
  // data_out_buffer = Uint32Array.from(new Uint32Array(gpu.buffers.read.getMappedRange()))
  gpu.buffers.read.unmap()
  if (LOOP_COMPUTE) {
    setTimeout(gpu_compute, 0);
  }
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


// const compute = () => {
//   computes.push({
//     time: performance.now()
//   })
//   while (computes.length > 100) {
//     computes.shift()
//   }
//   const cps_value = 1000/((computes[computes.length-1].time - computes[0].time) / computes.length)
//   byid("value_cps").innerHTML = cps_value.toFixed(1)
//   for (let i = 0 ; i < map_size*gravity_field_density ; i+=1) {
//     for (let j = 0 ; j < map_size*gravity_field_density ; j+=1) {
//       const gf_x = i / gravity_field_density;
//       const gf_y = j / gravity_field_density;
//       const id = gf_id(i,j)
//       gravity_field[id][0] = 0.0;
//       gravity_field[id][1] = 0.0;
//       const a = [gf_x, gf_y]
//       for (let part of parts) {
//         const b = [part.x, part.y]
//         const dv = min_dv(dvs(a,b))
//         const d_sqrd = distance_sqrd(dv)
//         const G = 1.0;
//         const f = G * mass * mass / d_sqrd
//         const d = Math.sqrt(d_sqrd)
//         const n = normalized(dv, d)
//         gravity_field[id][0] += f * n[0];
//         gravity_field[id][1] += f * n[1];
//       }
//     }
//   }
//   setTimeout(compute, 0);
// }


const draw = async () => {
  frames.push({
    time: performance.now()
  })
  while (frames.length > 100) {
    frames.shift()
  }
  const fps_value = 1000/((frames[frames.length-1].time - frames[0].time) / frames.length)
  byid("value_fps").innerHTML = fps_value.toFixed(1)
  context.fillStyle = "#222"

  context.fillRect(0, 0, canvas.width, canvas.height);
  const unit = canvas.width/map_size;



  const buffer_read = new DataView(data_out_buffer.buffer)
  if (DRAW_CIRCLES) {
    for (var j = 0; j < grid_width; j++) {
      for (var i = 0; i < grid_width; i++) {
        const cell_id = j * grid_width + i;
        const buffer_id = cell_id * attributs_count * float_size
        // const x = buffer_read.getFloat32(buffer_id,little_endian)
        // const y = buffer_read.getFloat32(buffer_id+float_size,little_endian)
        const p = {
          x: buffer_read.getFloat32(buffer_id,little_endian),
          y: buffer_read.getFloat32(buffer_id+float_size,little_endian),
          xx: buffer_read.getFloat32(buffer_id+float_size*2,little_endian),
          yy: buffer_read.getFloat32(buffer_id+float_size*3,little_endian),
        }
        p.x = (p.x + p.xx)*0.5
        p.y = (p.y + p.yy)*0.5
        // const pp = {
        //   x: buffer_read.getFloat32(buffer_id,little_endian),
        //   y: buffer_read.getFloat32(buffer_id+float_size,little_endian)
        // }
        const enabled = buffer_read.getInt32(buffer_id+float_size*4,little_endian)
        const debug = buffer_read.getInt32(buffer_id+float_size*5,little_endian)
        if (enabled === 1) {
          context.beginPath();
          context.arc(
            (canvas.width + canvas.width*0.5 + p.x*unit)%canvas.width,
            (canvas.width + canvas.width*0.5 - p.y*unit)%canvas.width,
            unit*0.5, 0, Math.PI * 2, true);
          context.fillStyle = "#8808"
          if (debug && DRAW_COLLISIONS) {
            context.fillStyle = "#f80"
          }
          context.fill()
        }
      }
    }
  }

  if (DRAW_SQUARES) {
    for (var j = 0; j < grid_width; j++) {
      for (var i = 0; i < grid_width; i++) {
        const cell_id = j * grid_width + i;
        const buffer_id = cell_id * attributs_count * float_size
        const x = buffer_read.getFloat32(buffer_id, little_endian)
        const y = buffer_read.getFloat32(buffer_id+float_size, little_endian)
        const x2 = i * 0.5 + 0.25
        const y2 = j * 0.5 + 0.25
        const enabled = buffer_read.getInt32(buffer_id+float_size*4,little_endian)
        const xx = (canvas.width + canvas.width*0.5 + x*unit)%canvas.width
        const yy = (canvas.width + canvas.width*0.5 - y*unit)%canvas.width
        const xx2 = (canvas.width + canvas.width*0.5 + x2*unit)%canvas.width
        const yy2 = (canvas.width + canvas.width*0.5 - y2*unit)%canvas.width
        if (enabled === 1) {
          context.fillStyle = "#a00"
          context.strokeStyle = "#d00"
          context.beginPath();
          context.arc(
            xx,
            yy,
            unit*0.05, 0, Math.PI * 2, true);
          context.fill()
          context.beginPath();
          context.moveTo(xx, yy);
          context.lineTo(xx2,yy2);
          context.stroke();
        }
        if (enabled === 2) {
          context.fillStyle = "#0f02"
          context.fillRect(xx2-unit*0.25, yy2-unit*0.25, unit*0.5, unit*0.5);
        }
      }
    }
  }



  context.strokeStyle = "#480"
  context.fillStyle = "#480"
  for (let i = 0 ; i < map_size*gravity_field_density ; i+=1) {
    for (let j = 0 ; j < map_size*gravity_field_density ; j+=1) {
      const x = i/gravity_field_density;
      const y = j/gravity_field_density;
      const id = gf_id(i,j)
      const xc = (canvas.width + canvas.width*0.5 + x*unit)%canvas.width
      const yc = (canvas.width + canvas.width*0.5 - y*unit)%canvas.width
      context.beginPath();
      context.moveTo(
        xc,
        yc);
      const dgf_nd = normalized(gravity_field[id])
      const dgfx = dgf_nd[0]*unit/(gravity_field_density+1)
      const dgfy = dgf_nd[1]*unit/(gravity_field_density+1)
      context.lineTo(
        xc + dgfx,
        yc - dgfy);
      context.stroke();
      context.beginPath();
      context.arc(
        xc + dgfx,
        yc - dgfy,
        unit*0.03, 0, Math.PI * 2, true);
      context.fill()
    }
  }


  context.fillStyle = "#333"
  for (let x = 1; x < grid_width; x++) {
    // context.fillRect(x/grid_width*canvas.width, 0, 1, canvas.height);
    // context.fillRect(0, x/grid_width*canvas.width, canvas.width, 1  );
  }
  context.fillStyle = "#666"
  for (let x = 1; x < map_width; x++) {
    // context.fillRect(x/map_width*canvas.width, 0, 1, canvas.height);
    // context.fillRect(0, x/map_width*canvas.width, canvas.width, 1  );
  }
  context.fillStyle = "#840"
  if (DRAW_ORIGIN) {
    context.fillRect(0.5*canvas.width, 0, 1, canvas.height);
    context.fillRect(0, 0.5*canvas.width, canvas.height, 1);
  }


  if (LOOP_DRAW) {
    requestAnimationFrame(() => {
      requestAnimationFrame(() => {
        draw()
      })
    })
  }
}

export {
  go
}
