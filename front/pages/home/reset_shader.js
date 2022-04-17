import {
  shader_common
} from './shader_common'

const reset_shader = (x) => { return `
// struct Cell {
//   p:  vec2<f32>;
//   pp: vec2<f32>;
//   enabled: i32;
//   debug: i32;
// };
// fn cell_id_fn(gid: vec2<u32>) -> u32 {
//   return gid.x + gid.y * ${x.grid_width}u ;
// }

${shader_common}


let NEIGHBOORS: i32 = 1;

// [[block]] struct Data {
//   cells: array<Cell, ${x.cell_count}>;
// };
[[group(0), binding(0)]] var<storage, write>  output    : Data;
[[stage(compute), workgroup_size(${x.workgroup_size}, ${x.workgroup_size})]]
fn main([[builtin(global_invocation_id)]] gid : vec3<u32>) {
  let cell_id = cell_id_fn(gid.xy);
  output.cells[cell_id].p.x = 0.0;
  output.cells[cell_id].p.y = 0.0;
  output.cells[cell_id].pp.x = 0.0;
  output.cells[cell_id].pp.y = 0.0;
  output.cells[cell_id].enabled = 0;
  output.cells[cell_id].debug = 0;
}`}

export {
  reset_shader
}
