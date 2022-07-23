import {
  COMPUTE_ITER,
  attributs_count,
  float_size,
  global_attributs_count,
  map_width,
  kind,
  little_endian,
  cell_count,
  LOOP_COMPUTE,
} from "./constants";
import {
  byid
} from "./dom";
import {
  add_part_2
} from "./add_parts";


const computes = []
const update_cps = () => {
  computes.push({
    time: performance.now()
  })
  while (computes.length > 100) {
    computes.shift()
  }
  const cps_value = 1000/((computes[computes.length-1].time - computes[0].time) / computes.length)
  byid("value_cps").innerHTML = cps_value.toFixed(1)
}


const buffer_size = (x) => {
  return (x.cell_count * attributs_count + global_attributs_count) * float_size;
}

let mousex = 0;
let mousey = 0;
let mouse_clicks = []



let from_write  = true
let gpu
let canvas
const gpu_compute = async (gpu_) => {
  gpu = gpu_
  canvas = document.getElementById("gpu_canvas")
  canvas.addEventListener("mousemove", () => {
    mousex = event.clientX;
    mousey = event.clientY;
  });
  canvas.addEventListener("mousedown", (e) => {
    mousex = event.clientX;
    mousey = event.clientY;
    mouse_clicks.push([mousex,mousey])
  });
  gpu_compute_inner()
}
let data_out_buffer;
const gpu_compute_inner = async () => {
  for (var i = 0; i < COMPUTE_ITER; i++) {
    update_cps()
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
      gpu.buffers.in, 0,
      buffer_size({cell_count:gpu.cell_count}));
    // command_encoder.copyBufferToBuffer(
    //   gpu.buffers.out, 0,
    //   gpu.buffers.write, 0,
    //   buffer_size({cell_count:gpu.cell_count}));
    command_encoder.copyBufferToBuffer(
      gpu.buffers.reset, 0,
      gpu.buffers.out, 0 ,
      buffer_size({cell_count:gpu.cell_count}));
    const gpu_commands = command_encoder.finish();
    gpu.device.queue.submit([gpu_commands]);
  }
  const mouse_x = mousex * map_width / canvas.width - map_width * 0.5
  const mouse_y = map_width * 0.5 - mousey * map_width / canvas.width
  byid("value_x").innerHTML = mouse_x.toFixed(1)
  byid("value_y").innerHTML = mouse_y.toFixed(1)
  await gpu.buffers.read.mapAsync(GPUMapMode.READ);
  await gpu.buffers.write.mapAsync(GPUMapMode.WRITE);
  data_out_buffer = Uint32Array.from(new Uint32Array(gpu.buffers.read.getMappedRange()))
  const buffer_write_ = new Uint32Array(gpu.buffers.write.getMappedRange());
  buffer_write_.set( data_out_buffer )
  const buffer_write = new DataView(buffer_write_.buffer)
  buffer_write.setFloat32( float_size * (cell_count*attributs_count + 2), mouse_x, little_endian)
  buffer_write.setFloat32( float_size * (cell_count*attributs_count + 3), mouse_y, little_endian)

  while (mouse_clicks.length > 0) {
    const click = mouse_clicks.shift()
    const x = click[0] * map_width / canvas.width - map_width * 0.5
    const y = map_width * 0.5 - click[1] * map_width / canvas.width
    add_part_2({
      xy: [x, y],
      dxy: [0.0, 0.0],
      static: false,
      mass: 1.0,
      buffer_write: buffer_write,
      map_width: map_width,
      kind: kind.miner
    })
  }

  from_write = true;
  gpu.buffers.read.unmap()
  gpu.buffers.write.unmap()
  if (LOOP_COMPUTE) {
    setTimeout(gpu_compute_inner, 1);
     //gpu_compute_inner();
  }
}


export {
  gpu_compute,
  buffer_size
}
