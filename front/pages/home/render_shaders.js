import {
  // float_size,
  // attributs_count,
  // little_endian,
  map_width,
  grid_width,
  // map_size,
  cell_count,
  LOOP_RENDER,
  kind,
} from "./constants"
import {
  shader_common
} from "./shader_common"


const fragment_shader = (a) => {return `
${shader_common}
[[group(0), binding(0)]] var<storage, read>   input     : Data;
[[stage(fragment)]]
fn main(  [[builtin(position)]] position: vec4<f32>    ) -> [[location(0)]] vec4<f32> {
  let x = position.x/f32(${a.canvas.width});
  let y = 1.0-position.y/f32(${a.canvas.height});
  var r = 0.1;
  var g = 0.1;
  var b = 0.1;

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
      let d = distance_wrap_around(cell.p, point_p);
      if (cell.enabled == 1 && d < 0.5 ) {
        if (cell.kind == ${kind.iron}) {
          r = 0.5;
          g = 0.5;
          b = 0.5;
        }
        elseif (cell.kind == ${kind.carbon}) {
          r = 0.25;
          g = 0.25;
          b = 0.25;
        }
        elseif (cell.kind == ${kind.water}) {
          r = 0.25;
          g = 0.25;
          b = 0.75;
        }
        elseif (cell.kind == ${kind.miner}) {
          r = 2.0 * d;
          g = 0.5;
          b = 0.5;

          if ( cell.debug == 7 ) {
            r = 1.0;
          }

        }
        else {
          r = 0.75;
          g = 0.0;
          b = 0.75;
        }
      }
    }
  }

  let cell_x = u32(point_p.x*2.0) % 64u;
  let cell_y = u32(point_p.y*2.0) % 64u;
  let cell = input.cells[cell_id_fn( vec2<u32>( cell_x, cell_y ))];

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


export {
  fragment_shader,
  vertex_shader
}
