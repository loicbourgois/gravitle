import {
  // float_size,
  // attributs_count,
  // little_endian,
  map_width,
  grid_width,
  // map_size,
  cell_count,
} from "./constants"

const fragment_shader = (a) => {return `
fn cell_id_fn(gid: vec2<u32>) -> u32 {
  return gid.x + gid.y * ${grid_width}u ;
}
struct Cell {
  p:  vec2<f32>;
  pp: vec2<f32>;
  enabled: i32;
  debug: i32;
  static: i32;
  mass: f32;
};
fn distance_wrap_around(a:vec2<f32>, b:vec2<f32>) -> f32{
  let o25 = f32(${map_width*0.25});
  let m25 = f32(${map_width*0.25});
  let o5 =  f32(${map_width*0.5});
  let m5 = f32(${map_width*0.5});
  let m = f32(${map_width});
  let a2 =   (vec2<f32>(   (a.x + o25+m)%m, (a.y + o25+m)%m  ));
  let b2 =   (vec2<f32>(   (b.x + o25+m)%m, (b.y + o25+m)%m  ));
  let a3 =   (vec2<f32>(   (a.x + o5+m)%m, (a.y + o5+m)%m  ));
  let b3 =   (vec2<f32>(   (b.x + o5+m)%m, (b.y + o5+m)%m  ));
  return min( min ( distance(a,b), distance(a2,b2) ), distance(a3,b3));
}
[[block]] struct Data {
  cells: array<Cell, ${a.cell_count}>;
};
[[group(0), binding(0)]] var<storage, read>   input     : Data;
[[stage(fragment)]]
fn main(  [[builtin(position)]] position: vec4<f32>    ) -> [[location(0)]] vec4<f32> {
  let x = position.x/f32(${a.canvas.width});
  let y = 1.0-position.y/f32(${a.canvas.height});
  var r = 0.0;
  var g = 0.0;
  var b = 0.0;

  let map_width = f32(${map_width});
  let grid_width = u32(${grid_width});

  let point_p = vec2<f32> (
    (x*map_width + map_width/2.0) % map_width,
    (y*map_width + map_width/2.0) % map_width,
  );

  let cell_x_start = u32(point_p.x*2.0) + grid_width - 1u;
  let cell_y_start = u32(point_p.y*2.0) + grid_width - 1u;

  for (var cell_x = cell_x_start ; cell_x < cell_x_start+3u ; cell_x =  cell_x + 1u) {
    for (var cell_y = cell_y_start ; cell_y < cell_y_start+3u ; cell_y =  cell_y + 1u) {

      let cell_x_ = (cell_x % grid_width);
      let cell_y_ = (cell_y % grid_width);
      let cell = input.cells[cell_id_fn( vec2<u32>( cell_x_, cell_y_ ))];

      if (distance_wrap_around(cell.p, point_p)  < 0.5 ) {
        r = 1.0;
        g = 1.0;
      }
    }

  }


  let cell_x = u32(point_p.x*2.0) % 64u;
  let cell_y = u32(point_p.y*2.0) % 64u;
  let cell = input.cells[cell_id_fn( vec2<u32>( cell_x, cell_y ))];




  if (cell.enabled != 1) {
    // b=1.0;
  }

  return vec4<f32>(r, g, b, 0.01);
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

  const render_ = () => {
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
    passEncoder.setBindGroup(0, bind_group);
    passEncoder.draw(6, 2, 0, 0);
    passEncoder.endPass();
    device.queue.submit([commandEncoder.finish()]);
    const LOOP_RENDER = true;
    if (LOOP_RENDER) {
      setTimeout(render_, 20);
    }
  }
render_()


}

export {
  gpu_render
}
