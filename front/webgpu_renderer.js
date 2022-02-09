import {
  last,
  len,
  assert,
} from "./util";
import * as render_shader from "./shaders/render";
import {materials} from "./materials";
import {
  update_fps
} from "./renderer_util";
async function render(x) {
  const start = performance.now();
  if (x.materials === undefined) {
    x.materials = materials
  }
  // Render
  const server_data = x.pull({
    player_id: x.player_id
  })
  if (server_data.buffer_size === undefined) {
    loop(x)
    return
  }
  x.grid_size = server_data.grid_size
  x.grid_width = server_data.grid_width
  x.grid_height = server_data.grid_height
  if (server_data.compute_duration) {
    document.getElementById("p_compute_duration").innerHTML = `Compute: ${server_data.compute_duration.toFixed(2)}ms`
  }
  if (server_data.cps) {
    document.getElementById("p_cps").innerHTML = `CPS: ${ server_data.cps.toFixed(1) } `
  }
  if (server_data.step) {
    document.getElementById("p_step").innerHTML = `Step: ${ server_data.step } `
  }
  if (x.image_size === undefined) {
    x.image_size = x.image_width * x.image_width
  }
  const canvas = document.getElementById("canvas");
  canvas.width  = x.image_width;
  canvas.height = x.image_height;
  canvas.style.width = window.innerWidth + "px";
  canvas.style.height = window.innerHeight + "px";
  const ctx = canvas.getContext("2d");
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
      // out_previous: x.device.createBuffer({
      //   size: image_buffer_size(x),
      //   usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST | GPUBufferUsage.COPY_SRC
      // }),
      read: x.device.createBuffer({
        size: image_buffer_size(x),
        usage: GPUBufferUsage.MAP_READ | GPUBufferUsage.COPY_DST
      })
    }
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
    x.dispatch = Math.ceil(x.image_width / x.workgroup_size);
  }
  if (x.compute_pipeline === undefined) {
    x.compute_pipeline = x.device.createComputePipeline({
      layout: x.device.createPipelineLayout({
        bindGroupLayouts: [x.bind_group_layout]
      }),
      compute: {
        module: x.device.createShaderModule({
          code: render_shader.get(x)
        }),
        entryPoint: "main"
      }
    });
  }
  // Render
  let write_map = x.buffers.write.mapAsync(GPUMapMode.WRITE);
  await write_map;
  new Uint32Array(x.buffers.write.getMappedRange()).set( server_data.buffer );
  //console.log(server_data.buffer)
  x.buffers.write.unmap()
  {
    const command_encoder = x.device.createCommandEncoder();
    command_encoder.copyBufferToBuffer(x.buffers.write, 0, x.buffers.in, 0 , server_data.buffer_size);
    const pass_encoder = command_encoder.beginComputePass();
    pass_encoder.setPipeline(x.compute_pipeline);
    pass_encoder.setBindGroup(0, x.bind_group);
    pass_encoder.dispatch(x.dispatch, x.dispatch);
    pass_encoder.endPass();
    command_encoder.copyBufferToBuffer(x.buffers.out, 0, x.buffers.read, 0 , image_buffer_size(x));
    const gpu_commands = command_encoder.finish();
    x.device.queue.submit([gpu_commands]);
  }
  const minimap = document.getElementById("minimap");
  if (minimap.width !== x.image_width) {
    minimap.width  = x.image_width;
  }
  if (minimap.height !== x.image_height) {
    minimap.height  = x.image_height;
  }
  const minimap_ctx = minimap.getContext("2d");
  let read_map = x.buffers.read.mapAsync(GPUMapMode.READ);
  await read_map;
  minimap_ctx.putImageData(
    new ImageData(
      Uint8ClampedArray.from(new Uint32Array(x.buffers.read.getMappedRange())),
      x.image_width,
      x.image_height,
    ),
    0, 0
  );
  x.buffers.read.unmap()
  update_fps(x)
  const end = performance.now();
  x.fps_counter.push({
    start: start,
    end: end,
    duration: end - start
  })
  loop(x)
}
function loop(x) {
  window.requestAnimationFrame(function () {
    render(x)
  })
}
function image_buffer_size(x) {
  const float_size = 4
  const pixel_size = 4
  return x.image_width * x.image_height * pixel_size * float_size
}
export {
  render
}
