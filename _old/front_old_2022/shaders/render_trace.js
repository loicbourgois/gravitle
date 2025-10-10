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
  kind: u32;
  ta: f32;
  tb: f32;
  tc: f32;
  td: f32;
  direction_x: f32;
  direction_y:  f32;
};
struct Kind {
  FIREFLY: u32;
  METAL: u32;
  TURBO: u32;
  DIATOM: u32;
  NEURON: u32;
  MOUTH:  u32;
  CORE:  u32;
  EGG:  u32;
};
let kind: Kind = Kind(1u, 2u, 3u, 4u, 5u, 6u, 7u, 8u);
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
  if (part_id <= u32(data.count))
  {
    let part = input.parts[part_id];
    var part_x = part.x;
    var part_y = part.y;
    if (part_x > data.x_max) {
      part_x = part_x - 1.0;
    }
    if (part_y > data.y_max) {
      part_y = part_y - 1.0;
    }

    if (
      data.x_min <= part_x && part_x <= data.x_max
      && data.y_min <= part_y && part_y <= data.y_max
    )
    {
      let x = part_x * zoom;
      let y = part_y * zoom;
      let cd_max = max(data.image_height, data.image_width);
      let d = max(min(0.1, part.d), 0.0001);
      let rayon  = d * 0.5 * zoom;
      let cd_min = min(data.image_height, data.image_width);
      let resolution = 0.00005 * zoom;
      let boo = cd_max * 0.5 * (zoom - 1.0) ;
      let bup_x = (0.5 - data.center_x) * zoom * cd_max - (cd_max - data.image_width) * 0.5 - boo;
      let bup_y = (0.5 - data.center_y) * zoom * cd_max - (cd_max - data.image_height)* 0.5 - boo;

      let i_min = -rayon;
      let i_max = rayon;
      let j_min = -rayon;
      let j_max = rayon;

      for (var i = i_min ; i < i_max ; i=i+resolution ) {
        for (var j = j_min ; j < j_max ; j=j+resolution ) {
          let xx = (x+i) * cd_max + bup_x ;
          let yy = (y+j) * cd_max + bup_y ;
          if (xx < data.image_width) {
            let pix_id = u32(xx) + u32(yy) * u32(data.image_width);
            // distance from center
            let dc = sqrt(i*i+j*j) / (d * zoom * 0.5);
            let angle_ = fract(1.75 - (0.5 - 0.5*j/abs(j) * acos(normalize(vec2<f32>(i, j)).x) * ${UNPI}));
            let angle = fract(angle_ + data.time * 0.00025);
            if ( dc < 1.0) {
              var r = 0.0;
              var g = 0.0;
              var b = 0.0;
              if (part.kind == kind.FIREFLY) {
                // output.pix[pix_id].r = 150u + u32(255.0 * dc );
                // output.pix[pix_id].g = u32(255.0 * sin(angle*100.0)*dc);
                // output.pix[pix_id].b = 0u;
                // output.pix[pix_id].a = 255u;
              } elseif (part.kind == kind.METAL) {
                // output.pix[pix_id].r = 100u;
                // output.pix[pix_id].g = 100u;
                // output.pix[pix_id].b = 200u;
                // output.pix[pix_id].a = 255u;
              } elseif (part.kind == kind.MOUTH) {
                r = 1.0;
                g = 1.0;
              } elseif (part.kind == kind.DIATOM) {
                // output.pix[pix_id].r = 0u;
                // output.pix[pix_id].g = 255u;
                // output.pix[pix_id].b = 200u;
                // output.pix[pix_id].a = 255u;
              } elseif (part.kind == kind.TURBO) {
                // r = 0.75 + part.ta ;
                // g = 0.75;
                r = 0.75;
                g = part.ta *   (1.0-sin(dc*0.7));
              } elseif (part.kind == kind.NEURON) {
                r = 0.25;
                g = part.ta *   (1.0-sin(dc*0.7));
                b = part.ta *   (1.0-sin(dc*0.7));
              } elseif (part.kind == kind.CORE) {
                if (part.ta < 1.0) {
                  r = 1.0 - part.ta;
                  g = part.ta;
                } else {
                  b = part.ta - 1.0;
                  g = 1.0;
                }
              } else {
                r = 1.0;
                b = 1.0;
              }
              output.pix[pix_id].r = u32(255.0 * r);
              output.pix[pix_id].g = u32(255.0 * g);
              output.pix[pix_id].b = u32(255.0 * b);
              output.pix[pix_id].a = 255u;
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
