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

fn random (st: vec2<f32>) -> f32 {
    return fract(sin(dot(st.xy,
                         vec2<f32>(19.9878,78.233)))*
        40908.5453123);
}

[[group(0), binding(0)]] var<storage, read>   input     : Data;
[[stage(fragment)]]
fn main(  [[builtin(position)]] position: vec4<f32>    ) -> [[location(0)]] vec4<f32> {
  let x = position.x/f32(${a.canvas.width});
  let y = 1.0-position.y/f32(${a.canvas.height});
  var r = 0.02;
  var g = 0.02;
  var b = 0.02;

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

        let uuu = f32(u32(d*13.0));
        let rrr = 0.5 + 0.5*random( vec2<f32>(uuu,uuu) );
        let m = min(1.0, 1.0-d);
        let dt = f32(cell.downtimer);

        switch (cell.kind) {
          case ${kind.iron}: {
            r = 0.75*m;
            g = 0.75*m;
            b = 0.85*m;
          }
          case ${kind.carbon}: {
            r = 0.25*m;
            g = 0.25*m;
            b = 0.25*m;
          }
          case ${kind.water}: {
            r = 0.55*m;
            g = 0.55*m;
            b = 1.15*m;
          }
          case ${kind.miner}: {
            r = 0.0;
            g = (1.25 - dt / 1000.0)*m;
            b = (1.0 - dt / 1000.0)*m;
            if ( cell.debug == 7 ) {
              r = 1.0;
            }
          }
          case ${kind.heater}: {
            r = 2.0 * m;
            g = 0.75 * m;
            b = 0.0;
            if ( cell.debug == 7 ) {
              r = 1.0*m;
            }
          }
          case ${kind.ice}: {
            r = 0.75*m;
            g = 0.95*m;
            b = 1.45*m;
          }
          case ${kind.stone}: {
            r = 0.65 * m;
            g = 0.45 * m;
            b = 0.45 * m;
          }
          case ${kind.launcher}: {
            r = (0.85 + 0.3 * f32(cell.downtimer)*0.02) * m;
            g = (0.4 + 0.3 * f32(cell.downtimer)*0.002) * m;
            b = (0.75 + 0.3 * f32(cell.downtimer)*0.002) * m;
          }
          default: {
            r = 0.75;
            g = 0.0;
            b = 0.75;
          }
        }
      }
    }
  }
  let cell_x = u32(point_p.x*2.0) % ${map_width}u;
  let cell_y = u32(point_p.y*2.0) % ${map_width}u;
  let cell = input.cells[cell_id_fn( vec2<u32>( cell_x, cell_y ))];



  if (distance_wrap_around(point_p,input.mouse) < 0.5 && distance_wrap_around(point_p,input.mouse) > 0.42 || distance_wrap_around(point_p,input.mouse) < 0.05) {
    r = 1.0;
    g = 1.0;
    b = 0.5;
  }

  return vec4<f32>(r, g, b, 1.0 );
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
