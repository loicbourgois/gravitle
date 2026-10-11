struct Cell {
     p: vec2f, // position
     pp: vec2f, // previous position
     ap: vec2f, // average position
     dp: vec2f, // delta position
     dv: vec2f, // delta velocity
     material_idx: u32,
     mass: f32,
     diameter: f32,
     fixed: u32,
     collision_count: f32,
     padding: u32,
}