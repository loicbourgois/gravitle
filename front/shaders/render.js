import {common} from "./common";
function get (x) {
  return `// Render shader
${common(x)}
struct Pixel {
  r: u32;
  g: u32;
  b: u32;
  a: u32;
};
[[block]] struct Image {
  pix: array<Pixel, ${x.image_size}>;
};
let unit = f32(${Math.floor(x.image_width / x.grid_width)});
fn fn_cell_id(gidx: u32, gidy: u32, zoom: f32) -> u32 {
  let x = f32(gidx) ;
  let y = f32(gidy);
  return u32 ( u32(x) / u32(unit) + u32(y) / u32(unit) * u32(${x.grid_width}) );
}
fn distance_(a:vec2<f32>, b:vec2<f32>) -> f32{
  let a2 =   fract(vec2<f32>(   (a.x + .25), (a.y + .25)  ));
  let b2 =   fract(vec2<f32>(   (b.x + .25), (b.y + .25)  ));
  let a3 =   fract(vec2<f32>(   (a.x + .5), (a.y + .5)  ));
  let b3 =   fract(vec2<f32>(   (b.x + .5), (b.y + .5)  ));
  return min( min ( distance(a,b), distance(a2,b2) ), distance(a3,b3));
}
[[group(0), binding(0)]] var<storage, read>   input     : Data;
[[group(0), binding(1)]] var<storage, write>  output    : Image;
[[stage(compute), workgroup_size(${x.workgroup_size}, ${x.workgroup_size})]]
fn main([[builtin(global_invocation_id)]] gid : vec3<u32>) {
  let zoom = 1.0;
  let cell_id = fn_cell_id(gid.x, gid.y, zoom);
  let pix_id = gid.x + gid.y * ${x.image_width}u;
  var coloring_cell_ids: array<u32, 9>;
  coloring_cell_ids[0] = up(left(cell_id));
  coloring_cell_ids[1] = up(cell_id);
  coloring_cell_ids[2] = right(up(cell_id));
  coloring_cell_ids[3] = left(cell_id);
  coloring_cell_ids[4] = cell_id;
  coloring_cell_ids[5] = right(cell_id);
  coloring_cell_ids[6] = left(down(cell_id));
  coloring_cell_ids[7] = down(cell_id);
  coloring_cell_ids[8] = right(down(cell_id));
  let pixel_point = vec2<f32>(f32(gid.x)/f32(${x.image_width}), f32(gid.y)/f32(${x.image_height}));
  var d_min = 999.0;
  var particle_kind = 0u;
  var particle_charge = 0.0;
  var particle_id = 0u;
  var ok = 0u;
  for (var i = 0 ; i < 9 ; i=i+1) {
    let particle_id_ = coloring_cell_ids[i];
    let particle = input.cells[particle_id_];
    let particle_center = vec2<f32>(
      particle.x,
      particle.y);
    let d = distance_( pixel_point, particle_center )*${x.grid_width}.0;
    if (input.cells[particle_id_].active == 1u && d < d_min) {
      d_min = d;
      particle_kind = particle.kind;
      particle_id = particle_id_;
      particle_charge = particle.charge;
      ok = 1u;
    }
  }
  let p = input.cells[particle_id];
  let u = ${1.0/x.grid_width};
  var in_radar = 0u;
  let x1 = f32(gid.x) / ${x.image_width}.0;
  let y1 = f32(gid.y) / ${x.image_height}.0;
  // for (var i = -radius_radar ; i <= radius_radar ;   i=i+u) {
  //   for (var j = -radius_radar ; j <= radius_radar ; j=j+u) {
  //     let x2 = x1 + i;
  //     let y2 = y1 + j;
  //     let cid = fn_cell_id(
  //       u32( fract(x2) * ${x.image_width}.0),
  //       u32( fract(y2) * ${x.image_height}.0),
  //       zoom);
  //     let c = input.cells[cid];
  //     let d = distance_(
  //       vec2<f32>(x1, y1),
  //       vec2<f32>(c.x, c.y)
  //     );
  //     if ( d <= f32(radius_radar) ) {
  //       if (c.kind == ${x.materials.RADAR}u && c.active == 1u) {
  //         in_radar = 1u;
  //         break;
  //       }
  //     }
  //   }
  //   if (in_radar == 1u) {
  //     break;
  //   }
  // }
  if (ok == 1u && d_min < 1.0) {
    output.pix[pix_id].a = 255u;
    if (particle_kind == ${x.materials.ROCK}u ) {
      output.pix[pix_id].r = 200u;
      output.pix[pix_id].g = 140u;
      output.pix[pix_id].b = 100u;
    } elseif (particle_kind == ${x.materials.WATER}u ) {
      output.pix[pix_id].r = 0u;
      output.pix[pix_id].g = 200u;
      output.pix[pix_id].b = 200u;
    } elseif (particle_kind == ${x.materials.CORE}u ) {
      output.pix[pix_id].r = 100u;
      output.pix[pix_id].g = 100u;
      output.pix[pix_id].b = 155u;
    } elseif (particle_kind == ${x.materials.METAL}u ) {
      output.pix[pix_id].r = 100u;
      output.pix[pix_id].g = 100u;
      output.pix[pix_id].b = 100u;
    } elseif (particle_kind == ${x.materials.TURBO}u ) {
      output.pix[pix_id].r = 255u;
      output.pix[pix_id].g = 100u;
      output.pix[pix_id].b = 0u;
    }
    elseif (particle_kind == ${x.materials.RADAR}u ) {
        output.pix[pix_id].r = 200u;
        output.pix[pix_id].g = 0u;
        output.pix[pix_id].b = 200u;
    }
    else {
      output.pix[pix_id].r = 255u;
      output.pix[pix_id].g = 0u;
      output.pix[pix_id].b = 255u;
    }
    if (d_min < 0.5) {
      if (particle_charge > 0.0) {
        output.pix[pix_id].r = 0u;
        output.pix[pix_id].g = 0u + u32(255.0 * particle_charge) ;
        output.pix[pix_id].b = 100u - u32(100.0 * particle_charge);
      } else {
        output.pix[pix_id].r = 0u + u32(255.0 * -particle_charge) ;
        output.pix[pix_id].g = 0u + u32(155.0 * -particle_charge) ;
        output.pix[pix_id].b = 100u - u32(100.0 * -particle_charge);
      }
    }
  }
  elseif (in_radar == 1u) {
    output.pix[pix_id].r = 100u;
    output.pix[pix_id].g = 50u;
    output.pix[pix_id].b = 20u;
    output.pix[pix_id].a = 255u;
  }
  elseif (gid.x == ${x.image_width/2}u || gid.y == ${x.image_height/2}u) {
    output.pix[pix_id].r = 0u;
    output.pix[pix_id].g = 255u;
    output.pix[pix_id].b = 0u;
    output.pix[pix_id].a = 255u;
  }
  else {
    output.pix[pix_id].r = 0u;
    output.pix[pix_id].g = 0u;
    output.pix[pix_id].b = 0u;
    output.pix[pix_id].a = 255u;
  }

  // if (p.debug > 0.5) {
  //   output.pix[pix_id].r = 255u;
  //   output.pix[pix_id].g = 0u;
  //   output.pix[pix_id].b = 0u;
  //   output.pix[pix_id].a = 255u;
  // }
}
`
}
export {
  get
}
