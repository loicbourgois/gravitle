import {
  map_width,
  grid_width,
  cell_count,
  LOOP_RENDER,
} from "./constants"


const shader_common = `
let max_speed = 0.25;


fn cell_id_fn(gid: vec2<u32>) -> u32 {
  return gid.x + gid.y * ${grid_width}u ;
}


fn cell_up(v: vec2<u32>) -> vec2<u32> {
  return vec2<u32>(v.x, (v.y + ${grid_width+1}u) % ${grid_width}u);
}
fn cell_down(v: vec2<u32>) -> vec2<u32> {
  return vec2<u32>(v.x, (v.y + ${grid_width-1}u) % ${grid_width}u);
}


fn delta_position_wrap_around(a:vec2<f32>, b:vec2<f32>) -> vec2<f32> {
  let o25 = f32(${map_width*0.25});
  let m25 = f32(${map_width*0.25});
  let o5 =  f32(${map_width*0.5});
  let m5 = f32(${map_width*0.5});
  let m = f32(${map_width});
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
  return vec2<u32>((v.x + ${grid_width-1}u) % ${grid_width}u, v.y);
}


fn cell_right(v: vec2<u32>) -> vec2<u32> {
  return vec2<u32>((v.x + ${grid_width+1}u) % ${grid_width}u, v.y);
}


struct Cell {
  p:  vec2<f32>;
  pp: vec2<f32>;
  enabled: i32;
  debug: i32;
  static: i32;
  mass: f32;
  kind: i32;
  downtimer: i32;
};


fn distance_wrap_around(a:vec2<f32>, b:vec2<f32>) -> f32{
  let o25 = f32(${map_width*0.25});
  let m25 = f32(${map_width*0.25});
  let o5 =  f32(${map_width*0.5});
  let m5 = f32(${map_width*0.5});
  let m = f32(${map_width});
  let a2 =   (vec2<f32>(   (a.x + o25+m)%m, (a.y + o25+m)%m  ));
  let b2 =   (vec2<f32>(   (b.x + o25+m)%m, (b.y + o25+m)%m  ));
  let a3 =   (vec2<f32>(   (a.x + o5+m)%m, (a.y + o5+m)%m  ));
  let b3 =   (vec2<f32>(   (b.x + o5+m)%m, (b.y + o5+m)%m  ));
  return min( min ( distance(a,b), distance(a2,b2) ), distance(a3,b3));
}


[[block]] struct Data {
  cells: array<Cell, ${cell_count}>;
  step: i32;
};
`


export {
    shader_common
}
