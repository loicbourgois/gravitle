const float_size = 4
const attributs_count = 10
const global_attributs_count = 4
const little_endian = true;
const map_width = 32;
const grid_width = map_width*2
const map_size = map_width;
const cell_count = grid_width * grid_width
const COMPUTE_ITER = 8
const LOOP_COMPUTE = true //&& false
const LOOP_RENDER = true //&& false
const kind = {
  invalid: 0,
  carbon: 1,
  stone: 2,
  iron: 3,
  water: 4,
  miner: 5,
  ice: 6,
  heater: 7,
  launcher: 8,
}


export {
  float_size,
  attributs_count,
  global_attributs_count,
  little_endian,
  map_width,
  grid_width,
  map_size,
  cell_count,
  COMPUTE_ITER,
  LOOP_COMPUTE,
  LOOP_RENDER,
  kind,
}
