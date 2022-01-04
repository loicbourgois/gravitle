// import {common} from "./common";
const UNPI = 1.0 / 3.1415926535897932384626433;
function get (x) {
  return `// Render shader
struct Part {
  x: f32;
  y: f32;
  x_old: f32;
  y_old: f32;
  d: f32;
  m: f32;
};
[[block]] struct Data {
  zoom: f32;
  center_x: f32;
  center_y: f32;
  count: f32;
  image_width: f32;
  image_height: f32;
  x_min: f32;
  y_min: f32;
  x_max: f32;
  y_max: f32;
  time: f32;
  step: f32;
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
[[group(0), binding(0)]] var<storage, read>   input     : Parts;
[[group(0), binding(1)]] var<storage, write>  output    : Image;
[[group(0), binding(2)]] var<storage, read>   data      : Data;
[[stage(compute), workgroup_size(${x.workgroup_size}, ${x.workgroup_size})]]
fn main([[builtin(global_invocation_id)]] gid : vec3<u32>) {
  let part_id = gid.x + gid.y * ${x.max_parts_sqrt}u;
  let zoom = min(10.0, data.zoom);
  if (part_id <= u32(data.count)) {
    let part = input.parts[part_id];
    if (data.x_min <= part.x && part.x <= data.x_max && data.y_min <= part.y && part.y <= data.y_max)
    {
      let x = part.x * zoom;
      let y = part.y * zoom;
      let cd_max = max(data.image_height, data.image_width);
      let d = max(min(0.1, part.d), 0.0001);
      let r  = d * 0.5 * zoom;
      let cd_min = min(data.image_height, data.image_width);
      let resolution = 0.0005 * zoom;
      let boo = cd_max * 0.5 * (zoom - 1.0) ;
      for (var i = -r ; i < r ; i=i+resolution ) {
        for (var j = -r ; j < r ; j=j+resolution ) {
          let xx = ( (x+i) * cd_max ) - (cd_max - data.image_width) * 0.5 - boo + (0.5 - data.center_x) * zoom * cd_max;
          let yy = ( (y+j) * cd_max ) - (cd_max - data.image_height)* 0.5 - boo + (0.5 - data.center_y) * zoom * cd_max;
          if (xx < data.image_width) {
            let pix_id = u32(xx) + u32(yy) * u32(data.image_width);
            // distance from center
            let dc = sqrt(i*i+j*j) / (d * zoom * 0.5);
            let angle = fract(1.75 - (0.5 - 0.5*j/abs(j) * acos(normalize(vec2<f32>(i, j)).x) * ${UNPI}));
            let angle_ = fract(angle + data.time * 0.00025);
            if ( dc < 1.0) {
              output.pix[pix_id].r = 150u + u32(255.0 * dc );
              output.pix[pix_id].b = 0u;
              output.pix[pix_id].a = 255u;
              output.pix[pix_id].g = u32(255.0* sin(angle_*100.0)*dc);
            }
          }
        }
      }
    }
  }
}
`
}
export {
  get
}
