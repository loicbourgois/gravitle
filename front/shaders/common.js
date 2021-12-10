function common (x) {
  const radius_radar = 8;
  return `
struct Link {
  active: u32;
  cell_id: u32;
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
`
}
export {
  common
}
