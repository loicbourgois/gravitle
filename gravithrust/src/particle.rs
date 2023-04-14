use crate::kind::Kind;
use crate::math::Vector;
use wasm_bindgen::prelude::wasm_bindgen;
pub type Particles = Vec<Particle>;
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Particle {
    pub p: Vector,
    pub v: Vector,
    pub pp: Vector,
    pub direction: Vector,
    pub m: f32,
    pub k: Kind,
    pub a: u32, // activated, usefull for boosters
    pub e: i32, // energy
    pub content: i32,
    pub grid_id: usize,
    pub idx: usize,
}
pub struct ParticleInternal {
    pub dp: Vector, // delta position
    pub dv: Vector, // delta velocity
    pub direction: Vector,
    pub sid: Option<usize>,
    pub new_kind: Vec<Kind>,
    pub new_content: Vec<i32>,
}
pub fn is_static(p: &Particle) -> bool {
    matches!(
        p.k,
        Kind::SunCore | Kind::Metal | Kind::Depot | Kind::Anchor
    )
}
pub fn do_collision(p: &Particle) -> bool {
    !matches!(p.k, Kind::Target | Kind::Anchor)
}
