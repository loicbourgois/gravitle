function common (x) {
  const radius_radar = 8;
  return `
struct Link {
  active: u32;
  cell_id: u32;
  weight: f32;
};
struct Cell {
  active: u32;
  kind: u32;
  x: f32;
  y: f32;
  x_old: f32;
  y_old: f32;
  mass: f32;
  entity_id: u32;
  charge: f32;
  debug: f32;
  cell_id_new: u32;
  links: array<Link, 6>;
};
[[block]] struct Data {
  cells: array<Cell, ${x.grid_size}>;
};
fn cell_id_fn(gid:vec2<u32>) -> u32 {
  return gid.x + gid.y * ${x.grid_width}u;
}
fn right (cell_id: u32) -> u32 {
  let x = cell_id % ${x.grid_width}u;
  let y = cell_id / ${x.grid_width}u;
  return (x+1u) % ${x.grid_width}u + y * ${x.grid_width}u;
}
fn left (cell_id: u32) -> u32 {
  let x = cell_id % ${x.grid_width}u;
  let y = cell_id / ${x.grid_width}u;
  return (x + ${x.grid_width}u - 1u) % ${x.grid_width}u + y * ${x.grid_width}u;
}
fn up (cell_id: u32) -> u32 {
  let x = cell_id % ${x.grid_width}u;
  let y = cell_id / ${x.grid_width}u;
  return x + ( (y + ${x.grid_width}u - 1u)%${x.grid_width}u ) * ${x.grid_width}u;
}
fn down (cell_id: u32) -> u32 {
  let x = cell_id % ${x.grid_width}u;
  let y = cell_id / ${x.grid_width}u;
  return x + ( (y + 1u)%${x.grid_height}u ) * ${x.grid_width}u;
}
let radius_radar_cells = ${radius_radar};
let radius_radar = ${radius_radar / x.grid_width};
fn distance_wrap_around(a:vec2<f32>, b:vec2<f32>) -> f32{
  let a2 =   (vec2<f32>(   fract(a.x + .25), fract(a.y + .25)  ));
  let b2 =   (vec2<f32>(   fract(b.x + .25), fract(b.y + .25)  ));
  let a3 =   (vec2<f32>(   fract(a.x + .5), fract(a.y + .5)  ));
  let b3 =   (vec2<f32>(   fract(b.x + .5), fract(b.y + .5)  ));
  return min( min ( distance(a,b), distance(a2,b2) ), distance(a3,b3));
}
fn delta_position_wrap_around(a:vec2<f32>, b:vec2<f32>) -> vec2<f32> {
  let a2 =   (vec2<f32>(   fract(a.x + .25), fract(a.y + .25)  ));
  let b2 =   (vec2<f32>(   fract(b.x + .25), fract(b.y + .25)  ));
  let a3 =   (vec2<f32>(   fract(a.x + .5), fract(a.y + .5)  ));
  let b3 =   (vec2<f32>(   fract(b.x + .5), fract(b.y + .5)  ));
  let d1 = distance(a,b);
  let d2 = distance(a2,b2);
  let d3 = distance(a3,b3);
  if (d1 < d2 ) {
    if (d1 < d3) {
      return a - b;
    } else {
     return a3 - b3;
    }
  }
  else{
    if (d2 < d3) {
      return a2 - b2;
    }
  }
  return a3 - b3;
}
let DIAMETER: f32 = ${2.0 / x.grid_width};
`
}
export {
  common
}
