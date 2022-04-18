import {
    kind
} from "./constants"
import {
  shader_common
} from "./shader_common"


const neighbours_setup = `
  var neighbouring_cells_xy: array<vec2<u32>, NEIGHBOURS>;
  neighbouring_cells_xy[0] = cell_up(gid.xy);
  neighbouring_cells_xy[1] = cell_up(neighbouring_cells_xy[0]);
  neighbouring_cells_xy[2] = cell_down(gid.xy);
  neighbouring_cells_xy[3] = cell_down(neighbouring_cells_xy[2]);
  neighbouring_cells_xy[4] = cell_left(gid.xy);
  neighbouring_cells_xy[5] = cell_left(neighbouring_cells_xy[0]);
  neighbouring_cells_xy[6] = cell_left(neighbouring_cells_xy[1]);
  neighbouring_cells_xy[7] = cell_left(neighbouring_cells_xy[2]);
  neighbouring_cells_xy[8] = cell_left(neighbouring_cells_xy[3]);
  neighbouring_cells_xy[9] = cell_right(gid.xy);
  neighbouring_cells_xy[10] = cell_right(neighbouring_cells_xy[0]);
  neighbouring_cells_xy[11] = cell_right(neighbouring_cells_xy[1]);
  neighbouring_cells_xy[12] = cell_right(neighbouring_cells_xy[2]);
  neighbouring_cells_xy[13] = cell_right(neighbouring_cells_xy[3]);
  neighbouring_cells_xy[14] = cell_left(neighbouring_cells_xy[4]);
  neighbouring_cells_xy[15] = cell_left(neighbouring_cells_xy[5]);
  neighbouring_cells_xy[16] = cell_left(neighbouring_cells_xy[6]);
  neighbouring_cells_xy[17] = cell_left(neighbouring_cells_xy[7]);
  neighbouring_cells_xy[18] = cell_left(neighbouring_cells_xy[8]);
  neighbouring_cells_xy[19] = cell_right(neighbouring_cells_xy[9]);
  neighbouring_cells_xy[20] = cell_right(neighbouring_cells_xy[10]);
  neighbouring_cells_xy[21] = cell_right(neighbouring_cells_xy[11]);
  neighbouring_cells_xy[22] = cell_right(neighbouring_cells_xy[12]);
  neighbouring_cells_xy[23] = cell_right(neighbouring_cells_xy[13]);
  var neighbouring_cell_ids: array<u32, NEIGHBOURS>;
  var neighbours: array<Cell, NEIGHBOURS>;
  for (var i = 0 ; i < NEIGHBOURS ; i=i+1) {
    neighbouring_cell_ids[i] = cell_id_fn(neighbouring_cells_xy[i]);
    neighbours[i] = input.cells[neighbouring_cell_ids[i]];
  }
`


const compute_shader_0 = (x) => { return `
${shader_common}


let NEIGHBOURS: i32 = 24;

[[group(0), binding(0)]] var<storage, read>   input     : Data;
[[group(0), binding(1)]] var<storage, write>  output    : Data;
[[stage(compute), workgroup_size(${x.workgroup_size}, ${x.workgroup_size})]]
fn main([[builtin(global_invocation_id)]] gid : vec3<u32>) {
  let cell_id = cell_id_fn(gid.xy);
  let p1 = input.cells[cell_id];
  if (p1.enabled == 1) {
    let velocity1 = p1.p - p1.pp;
    var forces = vec2<f32>(0.0, 0.0);
    var d_collision = vec2<f32>(0.0, 0.0);
    var d_collision_move = vec2<f32>(0.0, 0.0);
    var colliding = false;
    var out_direction = vec2<f32>(0.0, 0.0);
    var kind = p1.kind;

    var launchers = 0;
    var launchers_direction = vec2<f32>(0.0, 0.0);

    if (p1.static == 0) {
      ${neighbours_setup}
      for (var i = 0 ; i < NEIGHBOURS ; i=i+1) {
        let p2 = neighbours[i];
        if (p2.enabled == 1) {
          let d = distance_wrap_around(p1.p, p2.p);
          var delta_position = vec2<f32>(0.0, 0.0);
          if (d < 1.1) {
            delta_position = delta_position_wrap_around(p1.p, p2.p);
            out_direction = out_direction + delta_position;
          }

          if (d < 1.0) {



            if (p1.kind == ${kind.ice} && p2.kind == ${kind.heater}) {
              kind = ${kind.water};
            }

            if (p1.static == 0 && p2.kind == ${kind.launcher}) {
              launchers = launchers + 1;
              launchers_direction = launchers_direction + delta_position;
            }

            colliding = true;
            d_collision_move = d_collision_move + normalize(delta_position) * (1.0-d)*0.55 ;
            {
              // https://en.wikipedia.org/wiki/Elastic_collision#Two-dimensional_collision_with_two_moving_objects
              let velocity2 = p2.p-p2.pp;
              let delta_velocity = velocity1 - velocity2;
              let mass_2 = 1.0;
              let mass_1 = 1.0;
              let mass_factor = 2.0 * mass_2 / (mass_2 + mass_1);
              let dot_vp = dot(delta_velocity, delta_position);
              let distance_ = distance(vec2<f32>(0.0, 0.0), delta_position);
              let distance_squared = distance_ * distance_;
              let acceleration = delta_position * mass_factor * dot_vp / distance_squared;
              d_collision = d_collision - acceleration * 0.5;
            }
          };
        }
      }
      for (var id3 = 0u ; id3 < ${x.grid_width * x.grid_width}u ; id3 = id3 + 1u) {
        if (id3 != cell_id) {
          let p3 = input.cells[id3];
          if (p3.enabled == 1) {
            let G = .25;
            let d = distance_wrap_around(p1.p, p3.p);
            let d_sqrd = d * d;
            let f = G * p1.mass * p3.mass / d_sqrd;
            let delta_position = delta_position_wrap_around(p1.p, p3.p);
            let n = normalize(delta_position);
            forces = forces - n * f;
          }
        }
      }
    }
    let acceleration = forces / p1.mass;
    let delta_time = 0.01;
    var speed = velocity1
      + acceleration * delta_time * delta_time
      + d_collision;

    if (launchers >= 2 && p1.static == 0 && p1.kind != ${kind.launcher}) {
      speed = speed + normalize(launchers_direction) * 0.022;
    }

    let max_speed = 0.25;


    speed.x = min(max(speed.x, -max_speed), max_speed);
    speed.y = min(max(speed.y, -max_speed), max_speed);

    var new_x = (
        input.cells[cell_id].p.x
        + ${x.map_width}.0
        + d_collision_move.x
        + speed.x
      ) % ${x.map_width}.0;
    var new_y = (
        input.cells[cell_id].p.y
        + ${x.map_width}.0
        + d_collision_move.y
        + speed.y
      ) % ${x.map_width}.0;
    let new_gid = vec2<u32>(u32(new_x*2.0), u32(new_y*2.0));
    let new_cell_id = cell_id_fn(new_gid);
    output.cells[new_cell_id].p.x = new_x;
    output.cells[new_cell_id].p.y = new_y;
    output.cells[new_cell_id].pp.x = new_x - speed.x ;
    output.cells[new_cell_id].pp.y = new_y - speed.y ;
    output.cells[new_cell_id].enabled = 1;
    output.cells[new_cell_id].mass = p1.mass;
    output.cells[new_cell_id].static = p1.static;
    output.cells[new_cell_id].kind = kind;


    var can_produce = true;
    if (p1.kind == ${kind.miner} && distance(out_direction, vec2<f32>(0.0,0.0) ) < 0.95  ) {
      can_produce = false;
    }


    if (can_produce && p1.kind == ${kind.miner} && input.step % 1000 == 1 ) {
      out_direction = normalize(out_direction);
      let new_new_x = new_x + out_direction.x * 1.01;
      let new_new_y = new_y + out_direction.y * 1.01;
      let new_new_gid = vec2<u32>(u32(new_new_x*2.0), u32(new_new_y*2.0));
      let new_new_cell_id = cell_id_fn(new_new_gid);
      if (input.cells[new_new_cell_id].enabled == 0) {
        output.cells[new_new_cell_id].p.x = new_new_x;
        output.cells[new_new_cell_id].p.y = new_new_y;
        output.cells[new_new_cell_id].pp.x = new_new_x - out_direction.x*0.01;
        output.cells[new_new_cell_id].pp.y = new_new_y - out_direction.y*0.01;
        output.cells[new_new_cell_id].enabled = 1;
        output.cells[new_new_cell_id].mass = 1.0;
        output.cells[new_new_cell_id].static = 0;
        output.cells[new_new_cell_id].kind = ${kind.ice};
      }
    }


    if (!can_produce) {
      output.cells[new_cell_id].debug = 7;
    }
    elseif (colliding) {
      output.cells[new_cell_id].debug = 1;
    }


  }
  output.step = input.step + 1;
}`}
export {
  compute_shader_0
}
