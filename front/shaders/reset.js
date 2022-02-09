import {common} from "./common";
function get (x) {
  return `// Reset shader
${common(x)}
[[group(0), binding(0)]] var<storage, read>   input     : Data;
[[group(0), binding(1)]] var<storage, write>  output    : Data;
[[stage(compute), workgroup_size(${x.workgroup_size}, ${x.workgroup_size})]]
fn main([[builtin(global_invocation_id)]] gid : vec3<u32>) {
  let cell_id = cell_id_fn(gid.xy);
  output.cells[cell_id].active = 0u;
  output.cells[cell_id].charge = 0.0;
}
`
}
export {
  get
}
