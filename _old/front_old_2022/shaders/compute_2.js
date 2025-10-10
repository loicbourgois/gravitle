import {common} from "./common";
import {materials} from "../materials";
import {linkings} from "./linking";
import {inter_linkings} from "./interlinking";
import {kinds_count, links} from "./compute";
function get (x) {
  return `// Post Compute shader
${common(x)}
${linkings(kinds_count, links, materials)}
${inter_linkings(kinds_count, links, materials)}
[[group(0), binding(0)]] var<storage, read>   input     : Data;
[[group(0), binding(1)]] var<storage, write>  output    : Data;
[[stage(compute), workgroup_size(${x.workgroup_size}, ${x.workgroup_size})]]
fn main([[builtin(global_invocation_id)]] gid : vec3<u32>) {
  let cell_id = cell_id_fn(gid.xy);
  output.cells[cell_id] = input.cells[cell_id];
  let p1 = input.cells[cell_id];
  for (var k = 0 ; k < 6 ; k=k+1) {
    if (input.cells[cell_id].links[k].active == 1u) {
      let old_cell_id = input.cells[cell_id].links[k].cell_id;
      let new_cell_id = input.cells[old_cell_id].cell_id_new;
      output.cells[cell_id].links[k].cell_id = new_cell_id;
      let p2id = new_cell_id;
      let p2 = input.cells[p2id];
      //if (p1.active == 1u && p2.active == 1u && p2id != cell_id) {
        let d = distance_wrap_around(vec2<f32>(p1.x, p1.y), vec2<f32>(p2.x, p2.y)) ;
        if (d > DIAMETER * 1.5) {
          output.cells[cell_id].links[k].active = 0u;
        }
      //}
    }
  }
}
`
}
export {
  get,
  materials
}
