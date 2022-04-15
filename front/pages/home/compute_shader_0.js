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
struct Cell {
  p:  vec2<f32>;
  pp: vec2<f32>;
  enabled: i32;
  debug: i32;
  static: i32;
  mass: f32;
};

fn cell_id_fn(gid: vec2<u32>) -> u32 {
  return gid.x + gid.y * ${x.grid_width}u ;
}

fn cell_up(v: vec2<u32>) -> vec2<u32> {
  return vec2<u32>(v.x, (v.y + ${x.grid_width+1}u) % ${x.grid_width}u);
}
fn cell_down(v: vec2<u32>) -> vec2<u32> {
  return vec2<u32>(v.x, (v.y + ${x.grid_width-1}u) % ${x.grid_width}u);
}
fn distance_wrap_around(a:vec2<f32>, b:vec2<f32>) -> f32{
  let o25 = f32(${x.map_width*0.25});
  let m25 = f32(${x.map_width*0.25});
  let o5 =  f32(${x.map_width*0.5});
  let m5 = f32(${x.map_width*0.5});
  let m = f32(${x.map_width});
  let a2 =   (vec2<f32>(   (a.x + o25+m)%m, (a.y + o25+m)%m  ));
  let b2 =   (vec2<f32>(   (b.x + o25+m)%m, (b.y + o25+m)%m  ));
  let a3 =   (vec2<f32>(   (a.x + o5+m)%m, (a.y + o5+m)%m  ));
  let b3 =   (vec2<f32>(   (b.x + o5+m)%m, (b.y + o5+m)%m  ));
  return min( min ( distance(a,b), distance(a2,b2) ), distance(a3,b3));
}

fn delta_position_wrap_around(a:vec2<f32>, b:vec2<f32>) -> vec2<f32> {
  let o25 = f32(${x.map_width*0.25});
  let m25 = f32(${x.map_width*0.25});
  let o5 =  f32(${x.map_width*0.5});
  let m5 = f32(${x.map_width*0.5});
  let m = f32(${x.map_width});
  let a2 =   (vec2<f32>(   (a.x + o25+m)%m, (a.y + o25+m)%m  ));
  let b2 =   (vec2<f32>(   (b.x + o25+m)%m, (b.y + o25+m)%m  ));
  let a3 =   (vec2<f32>(   (a.x + o5+m)%m, (a.y + o5+m)%m  ));
  let b3 =   (vec2<f32>(   (b.x + o5+m)%m, (b.y + o5+m)%m  ));
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


fn cell_left(v: vec2<u32>) -> vec2<u32> {
  return vec2<u32>((v.x + ${x.grid_width-1}u) % ${x.grid_width}u, v.y);
}
fn cell_right(v: vec2<u32>) -> vec2<u32> {
  return vec2<u32>((v.x + ${x.grid_width+1}u) % ${x.grid_width}u, v.y);
}
let NEIGHBOURS: i32 = 24;
[[block]] struct Data {
  cells: array<Cell, ${x.cell_count}>;
};
[[group(0), binding(0)]] var<storage, read>   input     : Data;
[[group(0), binding(1)]] var<storage, write>  output    : Data;
[[stage(compute), workgroup_size(${x.workgroup_size}, ${x.workgroup_size})]]
fn main([[builtin(global_invocation_id)]] gid : vec3<u32>) {
  let cell_id = cell_id_fn(gid.xy);
  let cell = input.cells[cell_id];
  if (cell.enabled == 1) {
    let p1 = cell;
    let velocity1 = p1.p - p1.pp;
    var forces = vec2<f32>(0.0, 0.0);
    var d_collision = vec2<f32>(0.0, 0.0);
    var d_collision_move = vec2<f32>(0.0, 0.0);
    var colliding = false;
    if (cell.static == 0) {
      ${neighbours_setup}
      for (var i = 0 ; i < NEIGHBOURS ; i=i+1) {
        let p2 = neighbours[i];
        if (p2.enabled == 1) {
          let d = distance_wrap_around(cell.p, p2.p);
          if (d < 1.0) {
            colliding = true;
            let delta_position = delta_position_wrap_around(cell.p, p2.p);
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
              // if (linked) {
              //   dx_collision = dx_collision - acceleration.x * 0.5;
              //   dy_collision = dy_collision - acceleration.y * 0.5;
              // }
              // else {
              //   dx_collision = dx_collision - acceleration.x * 1.0;
              //   dy_collision = dy_collision - acceleration.y * 1.0;
              // }

            }
          };
        }
      }
      for (var id3 = 0u ; id3 < ${x.grid_width*x.grid_width}u ; id3 = id3+1u) {
        if (id3 != cell_id) {
          let p3 = input.cells[id3];
          if (p3.enabled == 1) {
            let G = .25;
            let mass = 1.0;
            let d = distance_wrap_around(cell.p, p3.p);
            let d_sqrd = d * d;
            let f = G * mass * mass / d_sqrd;
            let delta_position = delta_position_wrap_around(cell.p, p3.p);
            let n = normalize(delta_position);
            forces = forces - n * f;
          }
        }
      }
    }
    let p1_mass = 1.0;
    let acceleration = forces / p1_mass;
    let delta_time = 0.01;
    var speed = velocity1
      + acceleration * delta_time * delta_time
      + d_collision;
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
    if (colliding) {
      output.cells[new_cell_id].debug = 1;
    }

  }
}`}
export {
  compute_shader_0
}
