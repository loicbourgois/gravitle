/* tslint:disable */
/* eslint-disable */
/**
*/
export enum Kind {
  Armor = 1,
  Core = 2,
  Booster = 3,
  Sun = 4,
  Light = 5,
  Plant = 6,
  Metal = 7,
  Depot = 8,
  Target = 9,
}
/**
*/
export class Delta {
  free(): void;
/**
*/
  direction: Vector;
/**
*/
  p: Vector;
/**
*/
  sid?: number;
/**
*/
  v: Vector;
}
/**
*/
export class Gravithrust {
  free(): void;
/**
* @param {number} diameter
* @param {number} sub_steps
* @param {number} max_rotation_speed
* @param {number} grid_side
* @param {number} max_speed_at_target
* @param {number} forward_max_speed
* @param {number} forward_max_angle
* @param {number} slow_down_max_angle
* @param {number} slow_down_max_speed_to_target_ratio
* @param {number} ship_count
* @returns {Gravithrust}
*/
  static new(diameter: number, sub_steps: number, max_rotation_speed: number, grid_side: number, max_speed_at_target: number, forward_max_speed: number, forward_max_angle: number, slow_down_max_angle: number, slow_down_max_speed_to_target_ratio: number, ship_count: number): Gravithrust;
/**
* @param {Vector} p
* @param {number} k
* @param {number | undefined} sid
*/
  add_particle(p: Vector, k: number, sid?: number): void;
/**
* @param {ShipModel} ship_model
* @param {Vector} position
*/
  add_ship(ship_model: ShipModel, position: Vector): void;
/**
* @returns {number}
*/
  particles_size(): number;
/**
* @returns {number}
*/
  particle_size(): number;
/**
* @returns {number}
*/
  particle_size_(): number;
/**
* @returns {number}
*/
  particles_count(): number;
/**
* @returns {number}
*/
  particles(): number;
/**
* @returns {number}
*/
  ships_size(): number;
/**
* @returns {number}
*/
  ship_size(): number;
/**
* @returns {number}
*/
  ship_size_(): number;
/**
* @returns {number}
*/
  ships_count(): number;
/**
* @returns {number}
*/
  ships(): number;
/**
*/
  ticks(): void;
/**
*/
  diameter: number;
/**
*/
  forward_max_angle: number;
/**
*/
  forward_max_speed: number;
/**
*/
  max_rotation_speed: number;
/**
*/
  max_speed_at_target: number;
/**
*/
  points: number;
/**
*/
  slow_down_max_angle: number;
/**
*/
  slow_down_max_speed_to_target_ratio: number;
/**
*/
  step: number;
/**
*/
  sub_steps: number;
}
/**
*/
export class Link {
  free(): void;
}
/**
*/
export class Particle {
  free(): void;
/**
*/
  a: number;
/**
*/
  direction: Vector;
/**
*/
  grid_id: number;
/**
*/
  idx: number;
/**
*/
  k: number;
/**
*/
  m: number;
/**
*/
  p: Vector;
/**
*/
  pp: Vector;
/**
*/
  v: Vector;
}
/**
*/
export class ShipModel {
  free(): void;
}
/**
*/
export class Vector {
  free(): void;
/**
*/
  x: number;
/**
*/
  y: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_gravithrust_free: (a: number) => void;
  readonly __wbg_get_gravithrust_diameter: (a: number) => number;
  readonly __wbg_set_gravithrust_diameter: (a: number, b: number) => void;
  readonly __wbg_get_gravithrust_points: (a: number) => number;
  readonly __wbg_set_gravithrust_points: (a: number, b: number) => void;
  readonly __wbg_get_gravithrust_step: (a: number) => number;
  readonly __wbg_set_gravithrust_step: (a: number, b: number) => void;
  readonly __wbg_get_gravithrust_sub_steps: (a: number) => number;
  readonly __wbg_set_gravithrust_sub_steps: (a: number, b: number) => void;
  readonly __wbg_get_gravithrust_max_rotation_speed: (a: number) => number;
  readonly __wbg_set_gravithrust_max_rotation_speed: (a: number, b: number) => void;
  readonly __wbg_get_gravithrust_max_speed_at_target: (a: number) => number;
  readonly __wbg_set_gravithrust_max_speed_at_target: (a: number, b: number) => void;
  readonly __wbg_get_gravithrust_forward_max_speed: (a: number) => number;
  readonly __wbg_set_gravithrust_forward_max_speed: (a: number, b: number) => void;
  readonly __wbg_get_gravithrust_forward_max_angle: (a: number) => number;
  readonly __wbg_set_gravithrust_forward_max_angle: (a: number, b: number) => void;
  readonly __wbg_get_gravithrust_slow_down_max_angle: (a: number) => number;
  readonly __wbg_set_gravithrust_slow_down_max_angle: (a: number, b: number) => void;
  readonly __wbg_get_gravithrust_slow_down_max_speed_to_target_ratio: (a: number) => number;
  readonly __wbg_set_gravithrust_slow_down_max_speed_to_target_ratio: (a: number, b: number) => void;
  readonly gravithrust_new: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number) => number;
  readonly gravithrust_add_particle: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly gravithrust_add_ship: (a: number, b: number, c: number) => void;
  readonly gravithrust_particles_size: (a: number) => number;
  readonly gravithrust_particle_size: (a: number) => number;
  readonly gravithrust_particles_count: (a: number) => number;
  readonly gravithrust_particles: (a: number) => number;
  readonly gravithrust_ships_size: (a: number) => number;
  readonly gravithrust_ship_size: (a: number) => number;
  readonly gravithrust_ships_count: (a: number) => number;
  readonly gravithrust_ships: (a: number) => number;
  readonly gravithrust_ticks: (a: number) => void;
  readonly gravithrust_particle_size_: (a: number) => number;
  readonly gravithrust_ship_size_: (a: number) => number;
  readonly __wbg_vector_free: (a: number) => void;
  readonly __wbg_get_vector_x: (a: number) => number;
  readonly __wbg_set_vector_x: (a: number, b: number) => void;
  readonly __wbg_get_vector_y: (a: number) => number;
  readonly __wbg_set_vector_y: (a: number, b: number) => void;
  readonly __wbg_delta_free: (a: number) => void;
  readonly __wbg_get_delta_p: (a: number) => number;
  readonly __wbg_set_delta_p: (a: number, b: number) => void;
  readonly __wbg_get_delta_v: (a: number) => number;
  readonly __wbg_set_delta_v: (a: number, b: number) => void;
  readonly __wbg_get_delta_direction: (a: number) => number;
  readonly __wbg_set_delta_direction: (a: number, b: number) => void;
  readonly __wbg_get_delta_sid: (a: number, b: number) => void;
  readonly __wbg_set_delta_sid: (a: number, b: number, c: number) => void;
  readonly __wbg_link_free: (a: number) => void;
  readonly __wbg_shipmodel_free: (a: number) => void;
  readonly __wbg_particle_free: (a: number) => void;
  readonly __wbg_get_particle_p: (a: number) => number;
  readonly __wbg_set_particle_p: (a: number, b: number) => void;
  readonly __wbg_get_particle_v: (a: number) => number;
  readonly __wbg_set_particle_v: (a: number, b: number) => void;
  readonly __wbg_get_particle_pp: (a: number) => number;
  readonly __wbg_set_particle_pp: (a: number, b: number) => void;
  readonly __wbg_get_particle_direction: (a: number) => number;
  readonly __wbg_set_particle_direction: (a: number, b: number) => void;
  readonly __wbg_get_particle_m: (a: number) => number;
  readonly __wbg_set_particle_m: (a: number, b: number) => void;
  readonly __wbg_get_particle_k: (a: number) => number;
  readonly __wbg_set_particle_k: (a: number, b: number) => void;
  readonly __wbg_get_particle_a: (a: number) => number;
  readonly __wbg_set_particle_a: (a: number, b: number) => void;
  readonly __wbg_get_particle_grid_id: (a: number) => number;
  readonly __wbg_set_particle_grid_id: (a: number, b: number) => void;
  readonly __wbg_get_particle_idx: (a: number) => number;
  readonly __wbg_set_particle_idx: (a: number, b: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
