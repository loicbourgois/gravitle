import {
  cell_count,
  grid_width,
  map_width,
} from "./constants"
import {compute_shader_0} from "./compute_shader_0";
import {reset_shader} from "./reset_shader";
import {add_parts} from "./add_parts";
import {
  // gpu_compute,
  buffer_size,
} from "./gpu_compute";


const gpu_setup = async (gpu) => {
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

export {
  gpu_setup
}
