import {common} from "./common";
import {materials} from "../materials";
import {linkings} from "./linking";
import {inter_linkings} from "./interlinking";
const links = [
  // material,  material,     strengh,  inter-entity
  ['WATER',     'WATER',      1.0,      1],
  ['FIRE',      'FIRE',       1.0,      0],
  ['ELECTRIC',  'ELECTRIC',   1.0,      0],
  ['METAL',     'METAL',      9.8,      0],
  ['TURBO',     'TURBO',      2.0,      0],
  ['COCKPIT',   'COCKPIT',    5.0,      0],
  ['METAL',     'TURBO',      2.0,      0],
  ['METAL',     'COCKPIT',    2.8,      0],
  ['TURBO',     'COCKPIT',    3.8,      0],
  ['WOOD',      'LEAF',       0.8,      0],
  ['LEAF',      'LEAF',       0.0,      0],
  ['WOOD',      'WOOD',       2.8,      0],
  ['CORE',      'CORE',       9.8,      0],
  ['METAL',     'CORE',       9.8,      0],
  ['TURBO',     'CORE',       9.8,      0],
  ['METAL',     'RADAR',      9.8,      0],
  ['CORE',      'RADAR',      9.8,      0],
]
const kinds_count = Object.keys(materials).length;
function get (x) {
  return `// Compute shader
${common(x)}
${linkings(kinds_count, links, materials)}
${inter_linkings(kinds_count, links, materials)}
let delta_time = ${1.0 / 60.0};
[[group(0), binding(0)]] var<storage, read>   input     : Data;
[[group(0), binding(1)]] var<storage, write>  output    : Data;
[[stage(compute), workgroup_size(${x.workgroup_size}, ${x.workgroup_size})]]
fn main([[builtin(global_invocation_id)]] gid : vec3<u32>) {
  let cell_id = cell_id_fn(gid.xy);
  let p1 = input.cells[cell_id];
  if(p1.active == 1u) {
    var neighboor_cell_id: array<u32, 24>;
    neighboor_cell_id[0] = up(up(left(left(cell_id))));
    neighboor_cell_id[1] = up(up(left(cell_id)));
    neighboor_cell_id[2] = up(up(cell_id));
    neighboor_cell_id[3] = up(up(right(cell_id)));
    neighboor_cell_id[4] = up(up(right(right(cell_id))));
    neighboor_cell_id[5] = up(left(left(cell_id)));
    neighboor_cell_id[6] = up(left(cell_id));
    neighboor_cell_id[7] = up(cell_id);
    neighboor_cell_id[8] = up(right(cell_id));
    neighboor_cell_id[9] = up(right(right(cell_id)));
    neighboor_cell_id[10] = left(left(cell_id));
    neighboor_cell_id[11] = left(cell_id);
    neighboor_cell_id[12] = right(cell_id);
    neighboor_cell_id[13] = right(right(cell_id));
    neighboor_cell_id[14] = down(left(left(cell_id)));
    neighboor_cell_id[15] = down(left(cell_id));
    neighboor_cell_id[16] = down(cell_id);
    neighboor_cell_id[17] = down(right(cell_id));
    neighboor_cell_id[18] = down(right(right(cell_id)));
    neighboor_cell_id[19] = down(down(left(left(cell_id))));
    neighboor_cell_id[20] = down(down(left(cell_id)));
    neighboor_cell_id[21] = down(down(cell_id));
    neighboor_cell_id[22] = down(down(right(cell_id)));
    neighboor_cell_id[23] = down(down(right(right(cell_id))));
    var forces = vec2<f32>(0.0, 0.0);
    let velocity1 = vec2<f32>(p1.x, p1.y) - vec2<f32>(p1.x_old, p1.y_old);
    var atraction_move = vec2<f32>(0.0, 0.0);
    let atraction_move_factor = 1.0;
    var attractions = 0u;
    var dx_collision = 0.0;
    var dy_collision = 0.0;
    var linked_neighbours_delta = vec2<f32>(0.0, 0.0);

    var links = p1.links;

    for (var i = 0 ; i < 24 ; i=i+1) {
      let p2id = neighboor_cell_id[i];
      if (input.cells[p2id].active == 1u && p2id != cell_id) {
        let p2 = input.cells[p2id];
        let d = distance_wrap_around(vec2<f32>(p1.x, p1.y), vec2<f32>(p2.x, p2.y)) ;
        let delta_position = delta_position_wrap_around (vec2<f32>(p1.x, p1.y), vec2<f32>(p2.x, p2.y) );
        var link_strength = linking[p1.kind][p2.kind];
        if (p1.entity_id != p2.entity_id) {
          link_strength = link_strength * inter_linking[p1.kind][p2.kind];
        }


        if ( link_strength > 0.0001 && d < DIAMETER * 1.01 ) {
          for (var k = 0 ; k < 6 ; k=k+1) {
            if (links[k].active == 1u && links[k].cell_id == p2id) {
              break;
            } elseif (links[k].active == 0u) {
              links[k].active = 1u;
              links[k].cell_id = p2id;
              // links[k].weight = 1.0;
              break;
            }
          }
        }


        var linked = false;
        for (var k = 0 ; k < 6 ; k=k+1) {
          if (links[k].active == 1u && links[k].cell_id == p2id) {
            linked = true;
            break;
          }
        }


        if ( linked ) {
          linked_neighbours_delta = linked_neighbours_delta + delta_position;
        }
        if ( linked ) {
          attractions = attractions + 1u;
          forces = forces +  normalize(delta_position) *  (DIAMETER - d) * link_strength * 100.0;
        }
        if (d < DIAMETER ) {
          // https://en.wikipedia.org/wiki/Elastic_collision#Two-dimensional_collision_with_two_moving_objects
          let velocity2 = vec2<f32>(p2.x-p2.x_old, p2.y-p2.y_old);
          let delta_velocity = velocity1 - velocity2;
          let mass_2 = 1.0;
          let mass_factor = 2.0 * mass_2 / (p1.mass + p2.mass);
          let dot_vp = dot(delta_velocity, delta_position);
          let distance_ = distance(vec2<f32>(0.0, 0.0), delta_position);
          let distance_squared = distance_ * distance_;
          let acceleration = delta_position * mass_factor * dot_vp / distance_squared;
          if (linked) {
            dx_collision = dx_collision - acceleration.x * 0.5;
            dy_collision = dy_collision - acceleration.y * 0.5;
          }
          else {
            dx_collision = dx_collision - acceleration.x * 1.0;
            dy_collision = dy_collision - acceleration.y * 1.0;
          }
        }
      }
    }
    var charge = 0.0;
    if (p1.kind == ${x.materials.RADAR}u) {
      for (var i:i32 = -radius_radar_cells ; i <= radius_radar_cells ;   i=i+1) {
        for (var j:i32 = -radius_radar_cells ; j <= radius_radar_cells ; j=j+1) {
          let gid2 = vec2<u32>(
            u32( (i32(gid.x) + i + ${x.grid_width} ) % ${x.grid_width} ),
            u32( (i32(gid.y) + j + ${x.grid_height}) % ${x.grid_height} )
          );
          let p2id = cell_id_fn(gid2);
          let p2 = input.cells[p2id];
          let d = distance_wrap_around(vec2<f32>(p1.x, p1.y), vec2<f32>(p2.x, p2.y));
          output.cells[p2id].debug = 0.0;
          if (p1.entity_id != p2.entity_id && p2.active == 1u && d < radius_radar && d > 0.0) {
            output.cells[p2id].debug = 1.0;
            let charge_ = (d - radius_radar);
            // charge = max(charge, charge_);
            charge = max(charge, (radius_radar-d) / radius_radar) ;
          }
        }
      }
    } elseif (p1.kind == ${x.materials.CORE}u) {
      charge = 1.0;
    } else {
      var active_links = 0.0;
      for (var k = 0 ; k < 6 ; k=k+1) {
        if (links[k].active == 1u) {
          active_links = active_links + 1.0;
          charge = charge + links[k].weight * input.cells[links[k].cell_id].charge;
        }
      }
      charge = min(1.0, max(charge, -1.0));
    }


    let turbo = 0.0;
    if (p1.kind == ${x.materials.TURBO}u && charge != 0.0) {
       forces = forces - normalize(linked_neighbours_delta) * turbo * max(0.0, charge);
    }


    let acceleration = forces / p1.mass;
    let speed = vec2<f32>(p1.x, p1.y) - vec2<f32>(p1.x_old, p1.y_old)
      + acceleration * delta_time * delta_time
      + vec2<f32>(dx_collision, dy_collision);
    let x_old_ = p1.x;
    let y_old_ = p1.y;
    let x = fract(x_old_ + speed.x);
    let y = fract(y_old_ + speed.y);
    let x_old = x - speed.x;
    let y_old = y - speed.y;
    let cell_id_new = cell_id_fn(vec2<u32>(
      u32(x * ${x.grid_width}.0),
      u32(y * ${x.grid_height}.0)
    ));
    output.cells[cell_id_new].active    = p1.active;
    output.cells[cell_id_new].kind      = p1.kind;
    output.cells[cell_id_new].mass      = p1.mass;
    output.cells[cell_id_new].entity_id = p1.entity_id;
    output.cells[cell_id_new].links = links;
    output.cells[cell_id_new].x = x;
    output.cells[cell_id_new].y = y;
    output.cells[cell_id_new].charge = charge;
    output.cells[cell_id_new].x_old = x_old;
    output.cells[cell_id_new].y_old = y_old;
    output.cells[cell_id].cell_id_new = cell_id_new;
  }
}
`
}
export {
  get,
  materials,
  links,
  kinds_count,
}
