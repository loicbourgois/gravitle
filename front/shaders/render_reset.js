// import {common} from "./common";
function get (x) {
  return `// Render shader
struct Part {
  x: f32;
  y: f32;
  x_old: f32;
  y_old: f32;
  m: f32;
  d: f32;
};
[[block]] struct Data {
  zoom: f32;
  center_x: f32;
  center_y: f32;
};
[[block]] struct Parts {
  parts: array<Part>;
};
struct Pixel {
  r: u32;
  g: u32;
  b: u32;
  a: u32;
};
[[block]] struct Image {
  pix: array<Pixel, ${x.image_width * x.image_height}>;
};
// [[group(0), binding(0)]] var<storage, read>   input     : Parts;
[[group(0), binding(1)]] var<storage, write>  output    : Image;
[[stage(compute), workgroup_size(1, 1)]]
fn main([[builtin(global_invocation_id)]] gid : vec3<u32>) {
  let pix_id = gid.x + gid.y * ${x.image_width}u;
  output.pix[pix_id].r = 0u;
  output.pix[pix_id].g = 0u;
  output.pix[pix_id].b = 0u;
  output.pix[pix_id].a = 1u;
}
`
}
export {
  get
}
