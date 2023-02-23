// import {common} from "./common";
function get (x) {
  return `// Render shader
// struct Part {
//   x: f32;
//   y: f32;
//   x_old: f32;
//   y_old: f32;
//   m: f32;
//   d: f32;
//   d: f32;
// };
// [[block]] struct Data {
//   zoom: f32;
//   center_x: f32;
//   center_y: f32;
// };
// [[block]] struct Parts {
//   parts: array<Part>;
// };
struct Pixel {
  r: i32;
  g: i32;
  b: i32;
  a: i32;
};
[[block]] struct Image {
  pix: array<Pixel, ${x.image_width * x.image_height}>;
};
[[group(0), binding(0)]] var<storage, read>   input     : Image;
[[group(0), binding(1)]] var<storage, write>  output    : Image;
[[stage(compute), workgroup_size(2, 2)]]
fn main([[builtin(global_invocation_id)]] gid : vec3<u32>) {
  let pix_id = gid.x + gid.y * ${x.image_width}u;
  // output.pix[pix_id].r = max(input.pix[pix_id].r - 1u, 0u);
  // output.pix[pix_id].g = max(input.pix[pix_id].g - 1u, 0u);
  // output.pix[pix_id].b = max(input.pix[pix_id].b - 1u, 0u);

  let fade = 100;

  output.pix[pix_id].r = min(max(input.pix[pix_id].r - fade, 0), 255);
  output.pix[pix_id].g = min(max(input.pix[pix_id].g - fade, 0), 255);
  output.pix[pix_id].b = min(max(input.pix[pix_id].b - fade, 0), 255);

  output.pix[pix_id].a = 255;
}
`
}
export {
  get
}
