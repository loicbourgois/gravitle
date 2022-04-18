// const update cps
import {
  COMPUTE_ITER,
  attributs_count,
  float_size,
  LOOP_COMPUTE,
} from "./constants";
import {
  byid
} from "./dom";


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
  return (x.cell_count * attributs_count + 1) * float_size;
}


let from_write  = true
let gpu
const gpu_compute = async (gpu_) => {
  gpu = gpu_
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
    command_encoder.copyBufferToBuffer(
      gpu.buffers.reset, 0,
      gpu.buffers.out, 0 ,
      buffer_size({cell_count:gpu.cell_count}));
    const gpu_commands = command_encoder.finish();
    gpu.device.queue.submit([gpu_commands]);
  }
  await gpu.buffers.read.mapAsync(GPUMapMode.READ);
  data_out_buffer = Uint32Array.from(new Uint32Array(gpu.buffers.read.getMappedRange()))
  gpu.buffers.read.unmap()
  if (LOOP_COMPUTE) {
    setTimeout(gpu_compute_inner, 10);
    //gpu_compute_inner();
  }
}

export {
  gpu_compute,
  buffer_size
}
