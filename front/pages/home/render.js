import {
  map_width,
  grid_width,
  cell_count,
  LOOP_RENDER,
} from "./constants"
import {
  vertex_shader,
  fragment_shader,
} from "./render_shaders"
import {
  byid
} from "./dom"



const gpu_render = async (a) => {
  const adapter = a.adapter;
  const device = a.device;
  const canvas = document.getElementById("gpu_canvas")
  canvas.width = window.innerHeight;
  canvas.height = window.innerHeight;
  const context = canvas.getContext('webgpu');
  const presentationSize = [
    canvas.clientWidth,
    canvas.clientHeight,
  ];
  const presentationFormat = context.getPreferredFormat(adapter);
  context.configure({
    device,
    format: presentationFormat,
    size: presentationSize,
  });
  const bind_group_layout = device.createBindGroupLayout({
    entries: [
      { // In
        binding: 0,
        visibility: GPUShaderStage.FRAGMENT,
        buffer: {
          type: "storage"
        }
      },
    ]
  })
  const bind_group = device.createBindGroup({
    layout: bind_group_layout,
    entries: [
      {
        binding: 0,
        resource: {
          buffer: a.buffer
        }
      },
    ]
  })
  const pipeline = device.createRenderPipeline({
    vertex: {
      module: device.createShaderModule({
        code: vertex_shader,
      }),
      entryPoint: 'main',
    },
    fragment: {
      module: device.createShaderModule({
        code: fragment_shader({
          canvas: canvas,
          cell_count: cell_count
        }),
      }),
      entryPoint: 'main',
      targets: [
        {
          format: presentationFormat,
        },
      ],
    },
    primitive: {
      topology: 'triangle-list',
    },
    layout: device.createPipelineLayout({
      bindGroupLayouts: [bind_group_layout]
    }),
  });



  const frames = []
  const value_fps_element = byid("value_fps")
  const update_fps = () => {
    frames.push({
      time: performance.now()
    })
    while (frames.length > 100) {
      frames.shift()
    }
    const fps_value = 1000/((frames[frames.length-1].time - frames[0].time) / frames.length)
    value_fps_element.innerHTML = fps_value.toFixed(1)
  }


  const gpu_render_inner = () => {
    update_fps()
    const commandEncoder = device.createCommandEncoder();
    const renderPassDescriptor = {
      colorAttachments: [
        {
          view: context.getCurrentTexture().createView(),
          clearValue: { r: 0.0, g: 0.0, b: 0.0, a: 0.0 },
          loadValue: [0.2, 0.3, 0.5, 0.1],
          loadOp: 'clear',
          storeOp: 'store',
        },
      ],
    };
    const passEncoder = commandEncoder.beginRenderPass(renderPassDescriptor);
    passEncoder.setPipeline(pipeline);
    passEncoder.setBindGroup(0, bind_group);
    passEncoder.draw(6, 2, 0, 0);
    passEncoder.endPass();
    device.queue.submit([commandEncoder.finish()]);
    if (LOOP_RENDER) {
      // setTimeout(gpu_render_inner, 15);
      requestAnimationFrame(() => {
        requestAnimationFrame(() => {
          gpu_render_inner()
        })
      })
    }
  }
  gpu_render_inner()
}


export {
  gpu_render
}
