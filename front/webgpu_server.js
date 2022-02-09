import {
  last,
  len,
  assert,
} from "./util";
import * as compute_shader_reset from "./shaders/reset";
import * as compute_shader from "./shaders/compute";
import * as compute_shader_2 from "./shaders/compute_2";
import {materials} from "./materials";
const data_out = {}
const data_in = []
const little_endian = true;
let i = 0;
const CELL_FIELD_ACTIVE = i++;
const CELL_FIELD_KIND = i++;
const CELL_FIELD_X = i++;
const CELL_FIELD_Y = i++;
const CELL_FIELD_X_OLD = i++;
const CELL_FIELD_Y_OLD = i++;
const CELL_FIELD_MASS = i++;
const CELL_FIELD_ENTITY_ID = i++;
const CELL_FIELD_CHARGE = i++;
const CELL_FIELD_DEBUG = i++;
const CELL_FIELD_CELL_ID_NEW = i++;
const CELL_FIELD_LINKS = i;
const links_max = 6;
const link_fields = 3;
const CELL_ATTRIBUTS_COUNT = i + links_max*link_fields;
assert (CELL_ATTRIBUTS_COUNT === 11+links_max*link_fields && CELL_FIELD_ACTIVE === 0 && CELL_FIELD_LINKS === 11)
function pull (x) {
  return data_out
}
function add_particle(x) {
  x.x = Math.abs(x.x) % 1.0;
  x.y = Math.abs(x.y) % 1.0;
  const x_id = Math.floor(x.x * x.grid_width   );
  const y_id = Math.floor(x.y * x.grid_height  );
  const cell_id = Math.floor( (x_id + y_id * x.grid_width) * x.cell_attributs_count * 4 );
  assert (cell_id >= 0 && cell_id <= x.grid_size * x.cell_attributs_count * 4)
  // console.log(cell_id)
  // if (!dx) {
  //   dx = 0.0;
  // }
  // if (!dy) {
  //   dy = 0.0;
  // }
  x.buffer.setUint32(cell_id  + CELL_FIELD_ACTIVE * 4,    1,            little_endian)
  x.buffer.setUint32(cell_id  + CELL_FIELD_KIND * 4,      x.kind,       little_endian)
  x.buffer.setFloat32(cell_id + CELL_FIELD_X * 4,         x.x,          little_endian)
  x.buffer.setFloat32(cell_id + CELL_FIELD_Y * 4,         x.y,          little_endian)
  x.buffer.setFloat32(cell_id + CELL_FIELD_X_OLD * 4,     x.x - x.dx,   little_endian)
  x.buffer.setFloat32(cell_id + CELL_FIELD_Y_OLD * 4,     x.y - x.dy,   little_endian)
  x.buffer.setFloat32(cell_id + CELL_FIELD_MASS * 4,      x.mass,       little_endian)
  x.buffer.setUint32(cell_id + CELL_FIELD_ENTITY_ID * 4,  x.entity_id,  little_endian)
  x.buffer.setFloat32(cell_id + CELL_FIELD_CHARGE * 4,    0.0,          little_endian)
  x.buffer.setFloat32(cell_id + CELL_FIELD_DEBUG * 4,     0.0,          little_endian)

  for (let i = 0; i < links_max; i++) {
    x.buffer.setUint32(cell_id + (CELL_FIELD_LINKS + i * link_fields + 0) * 4, 0, little_endian)
    x.buffer.setUint32(cell_id + (CELL_FIELD_LINKS + i * link_fields + 1) * 4, 0, little_endian)
    x.buffer.setFloat32(cell_id + (CELL_FIELD_LINKS + i * link_fields + 2) * 4, Math.random()*2.0-1.0, little_endian)
  }

}
function add_ship(x) {
  for (let particle of ship({
      x: x.x,
      y: x.y,
      grid_width: x.grid_width,
      plan: x.plan
    })) {
      add_particle({
        buffer: x.buffer_write,
        x: particle.x,
        y: particle.y,
        grid_width: x.grid_width,
        grid_height: x.grid_height,
        cell_attributs_count: x.cell_attributs_count,
        grid_size: x.grid_size,
        dx: 0.00,
        dy: -0.000,
        mass: 1.0,
        kind: particle.material,
        entity_id: x.entity_id
      })
  }
}
async function serve(x) {
  const start = performance.now();
  if (x.step === undefined) {
    x.step = 0
  }
  if (x.cps_counter === undefined) {
    x.cps_counter = []
  }
  if (x.cell_attributs_count === undefined) {
    x.cell_attributs_count = CELL_ATTRIBUTS_COUNT;
  }
  if (x.grid_size === undefined) {
    x.grid_size = x.grid_width * x.grid_height
  }
  if (x.entity_id_max === undefined) {
    x.entity_id_max = -1
  }
  if (x.materials === undefined) {
    x.materials = materials
  }
  if (x.device === undefined) {
    if (! ("gpu" in navigator) ) {
      const m = "Photosonic requires WebGPU.\nInstructions on how to enable at https://web.dev/gpu/#use"
      alert(m)
      console.error(m)
      return
    }
    const adapter = await navigator.gpu.requestAdapter();
    if (!adapter) {
      console.error("No gpu adapter found")
      return;
    }
    x.device = await adapter.requestDevice();
  }
  if (x.buffers === undefined) {
    x.buffers = {
      write: x.device.createBuffer({
        size: buffer_size(x),
        usage: GPUBufferUsage.MAP_WRITE | GPUBufferUsage.COPY_SRC
      }),
      in: x.device.createBuffer({
        size: buffer_size(x),
        usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST | GPUBufferUsage.COPY_SRC
      }),
      out: x.device.createBuffer({
        size: buffer_size(x),
        usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST | GPUBufferUsage.COPY_SRC
      }),
      read: x.device.createBuffer({
        size: buffer_size(x),
        usage: GPUBufferUsage.MAP_READ | GPUBufferUsage.COPY_DST
      })
    }
    await x.buffers.write.mapAsync(GPUMapMode.WRITE);
    const buffer_write = new DataView(x.buffers.write.getMappedRange())
    // for (var i = 0; i < x.grid_width * x.grid_height / 1000 ; i++) {
    //   add_particle({
    //     buffer: buffer_write,
    //     x: Math.random() * 0.5,
    //     y: Math.random() * 0.5,
    //     grid_width: x.grid_width,
    //     grid_height: x.grid_height,
    //     cell_attributs_count: x.cell_attributs_count,
    //     grid_size: x.grid_size,
    //     dx: (Math.random() - 0.5) * 2.0 / x.grid_width * 0.1,
    //     dy: (Math.random() - 0.5) * 2.0 / x.grid_height * 0.1,
    //     mass: 1.0,
    //     kind: materials.ROCK
    //   })
    // }
    x.buffers.write.unmap()
  }
  if (x.bind_group_layout === undefined) {
    x.bind_group_layout = x.device.createBindGroupLayout({
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
    });
  }
  if (x.bind_group === undefined) {
    x.bind_group = x.device.createBindGroup({
      layout: x.bind_group_layout,
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
        },
      ]
    });
  }
  if (x.workgroup_size === undefined) {
    x.workgroup_size = 8
  }
  if (x.dispatch === undefined) {
    x.dispatch = Math.ceil(x.grid_width / x.workgroup_size);
  }
  if (x.compute_pipeline_reset === undefined) {
    x.compute_pipeline_reset = x.device.createComputePipeline({
      layout: x.device.createPipelineLayout({
        bindGroupLayouts: [x.bind_group_layout]
      }),
      compute: {
        module: x.device.createShaderModule({
          code: compute_shader_reset.get(x)
        }),
        entryPoint: "main"
      }
    });
  }
  if (x.compute_pipeline === undefined) {
    x.compute_pipeline = x.device.createComputePipeline({
      layout: x.device.createPipelineLayout({
        bindGroupLayouts: [x.bind_group_layout]
      }),
      compute: {
        module: x.device.createShaderModule({
          code: compute_shader.get(x)
        }),
        entryPoint: "main"
      }
    });
  }
  if (x.compute_pipeline_2 === undefined) {
    x.compute_pipeline_2 = x.device.createComputePipeline({
      layout: x.device.createPipelineLayout({
        bindGroupLayouts: [x.bind_group_layout]
      }),
      compute: {
        module: x.device.createShaderModule({
          code: compute_shader_2.get(x)
        }),
        entryPoint: "main"
      }
    });
  }
  // Compute
  x.step = x.step + 1
  {
    let map_buffer_write = false;
    for (const d of data_in) {
      if (d.command === 'add_ship') {
        map_buffer_write = true;
        break;
      } else if (d.command === 'add_particle') {
        map_buffer_write = true;
        break;
      } else {
        console.warn(`invalid command ${d.command}`)
      }
    }
    let buffer_write = undefined;
    if (map_buffer_write) {
      await x.buffers.write.mapAsync(GPUMapMode.WRITE);
      buffer_write = new DataView(x.buffers.write.getMappedRange())
    }
    while (data_in.length) {
      const d = data_in.pop();
      if (d.command === 'add_ship') {
        x.entity_id_max += 1;
        console.log(x.entity_id_max)
        let x_ = Math.random()
        let y_ = Math.random()
        if (d.x !== undefined) {
          x_ = d.x
        }
        if (d.y !== undefined) {
          y_ = d.y
        }
        console.log(d)
        add_ship({
          x: x_,
          y: y_,
          plan: d.plan,
          buffer_write: buffer_write,
          grid_width: x.grid_width,
          grid_height: x.grid_height,
          cell_attributs_count: x.cell_attributs_count,
          grid_size: x.grid_size,
          entity_id: x.entity_id_max
        })
      } else if (d.command === 'add_particle') {
        add_particle({
          buffer: buffer_write,
          x: d.x,
          y: d.y,
          grid_width: x.grid_width,
          grid_height: x.grid_height,
          cell_attributs_count: x.cell_attributs_count,
          grid_size: x.grid_size,
          dx: d.dx,
          dy: d.dy,
          mass: 1.0,
          kind: x.materials[d.kind],
          entity_id: 12,
        })
      }
    }
    if (map_buffer_write) {
      x.buffers.write.unmap()
    }
  }
  {
    const command_encoder = x.device.createCommandEncoder();
    const pass_encoder = command_encoder.beginComputePass();
    pass_encoder.setPipeline(x.compute_pipeline_reset);
    pass_encoder.setBindGroup(0, x.bind_group);
    pass_encoder.dispatch(x.dispatch, x.dispatch);
    pass_encoder.endPass();
    // command_encoder.copyBufferToBuffer(x.buffers.out, 0, x.buffers.in, 0 , buffer_size(x));
    const gpu_commands = command_encoder.finish();
    x.device.queue.submit([gpu_commands]);
  }
  {
    const command_encoder = x.device.createCommandEncoder();
    if (x.step == 1) {
      command_encoder.copyBufferToBuffer(x.buffers.write, 0, x.buffers.in, 0 , buffer_size(x));
    }
    const pass_encoder = command_encoder.beginComputePass();
    pass_encoder.setPipeline(x.compute_pipeline);
    pass_encoder.setBindGroup(0, x.bind_group);
    pass_encoder.dispatch(x.dispatch, x.dispatch);
    pass_encoder.endPass();
    command_encoder.copyBufferToBuffer(x.buffers.out, 0, x.buffers.in, 0 , buffer_size(x));
    //command_encoder.copyBufferToBuffer(x.buffers.out, 0, x.buffers.read, 0 , buffer_size(x));
    const gpu_commands = command_encoder.finish();
    x.device.queue.submit([gpu_commands]);
  }
  {
    const command_encoder = x.device.createCommandEncoder();
    const pass_encoder = command_encoder.beginComputePass();
    pass_encoder.setPipeline(x.compute_pipeline_2);
    pass_encoder.setBindGroup(0, x.bind_group);
    pass_encoder.dispatch(x.dispatch, x.dispatch);
    pass_encoder.endPass();
    command_encoder.copyBufferToBuffer(x.buffers.out, 0, x.buffers.in, 0 , buffer_size(x));
    command_encoder.copyBufferToBuffer(x.buffers.out, 0, x.buffers.read, 0 , buffer_size(x));
    const gpu_commands = command_encoder.finish();
    x.device.queue.submit([gpu_commands]);
  }
  // Data
  data_out.step = x.step
  data_out.buffer_size = buffer_size(x)
  data_out.grid_size = x.grid_size
  data_out.grid_width = x.grid_width
  data_out.grid_height = x.grid_height
  if (x.step % 1 === 0) {
    await x.buffers.read.mapAsync(GPUMapMode.READ);
    data_out.buffer = Uint32Array.from(new Uint32Array(x.buffers.read.getMappedRange()))
    x.buffers.read.unmap()
  }
  // CPS
  update_cps(x)
  const end = performance.now();
  x.cps_counter.push({
    start: start,
    end: end,
    duration: end - start
  })
  // Loop
  window.setTimeout(async function(){
    await serve(x);
  }, x.interval)
}
function buffer_size(x) {
  const float_size = 4
  return x.grid_width * x.grid_height * x.cell_attributs_count * float_size
}
function update_cps(x) {
  if (len(x.cps_counter) > 0) {
    while (len(x.cps_counter) > x.cps_counter_length) {
      x.cps_counter.shift()
    }
    let compute_duration = 0.0;
    for (let f of x.cps_counter) {
      compute_duration += f.duration;
    }
    compute_duration /= len(x.cps_counter)
    data_out.compute_duration = compute_duration
    data_out.cps = (len(x.cps_counter) / (last(x.cps_counter).end - x.cps_counter[0].start) * 1000)
  }
}
function ship(x) {
  const u = 2.0 / x.grid_width
  let particles = [{
    x: x.x - u * 0.5,
    y: x.y,
    material: materials.CORE
  },{
    x: x.x + u * 0.5,
    y: x.y,
    material: materials.CORE
  }]
  for (let p_ of x.plan) {
    let p = p_coords({
      p1: {
        x: particles[p_[0]].x,
        y: particles[p_[0]].y,
      },
      p2: {
        x: particles[p_[1]].x,
        y: particles[p_[1]].y,
      }
    })
    particles.push({
      x: p.x,
      y: p.y,
      material: materials[p_[2]]
    })
  }
  return particles
}
function p_coords(x) {
  return rotate({
    p1: x.p1,
    p2: x.p2,
    angle: Math.PI / 3.0 * 5.0,
  })
}
function rotate(x) {
  // Rotates p2 around p1
  const dx = x.p2.x - x.p1.x;
  const dy = x.p2.y - x.p1.y;
  const cos_ = Math.cos(x.angle);
  const sin_ = Math.sin(x.angle);
  return {
    x: x.p1.x + dx*cos_ - dy*sin_,
    y: x.p1.y + dy*cos_ + dx*sin_
  }
}
function push(x) {
  data_in.push(x)
}
export {
  pull,
  serve,
  push
}
