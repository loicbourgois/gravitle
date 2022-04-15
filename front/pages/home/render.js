const fragment_shader = (a) => {return `
[[stage(fragment)]]
fn main(  [[builtin(position)]] position: vec4<f32>    ) -> [[location(0)]] vec4<f32> {
  let x = position.x/f32(${a.canvas.width});
  let y = 1.0-position.y/f32(${a.canvas.height});
  let r = x;
  let g = y;
  var b = 0.0;
  return vec4<f32>(x, y, b, 0.01);
}
`};


const vertex_shader = `
let pos = array<vec2<f32>, 6>(
    vec2<f32>(-1.0, 1.0),
    vec2<f32>(1.0, 1.0),
    vec2<f32>(-1.0, -1.0),
    vec2<f32>(1.0, 1.0),
    vec2<f32>(-1.0, -1.0),
    vec2<f32>(1.0, -1.0),
);
[[stage(vertex)]]
fn main( [[builtin(vertex_index)]] vertex_index : u32) -> [[builtin(position)]] vec4<f32> {
  var pos_ = pos;
  return vec4<f32>(pos_[vertex_index], 0.0, 1.0);
}
`

const gpu_render = async () => {
  const adapter = await navigator.gpu.requestAdapter();
  const device = await adapter.requestDevice();
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
  });
  {
    const commandEncoder = device.createCommandEncoder();
    const textureView = context.getCurrentTexture().createView();
    const renderPassDescriptor = {
      colorAttachments: [
        {
          view: textureView,
          clearValue: { r: 0.0, g: 0.0, b: 0.0, a: 0.0 },
          loadValue: [0.2, 0.3, 0.5, 0.1],
          loadOp: 'clear',
          storeOp: 'store',
        },
      ],
    };
    const passEncoder = commandEncoder.beginRenderPass(renderPassDescriptor);
    passEncoder.setPipeline(pipeline);
    passEncoder.draw(6, 2, 0, 0);
    passEncoder.endPass();
    device.queue.submit([commandEncoder.finish()]);
  }
}

export {
  gpu_render
}
