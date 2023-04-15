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
    pub e: u32, // energy
    pub volume: u32,
    pub grid_id: usize,
    pub idx: usize,
}
pub struct ParticleInternal {
    pub dp: Vector, // delta position
    pub dv: Vector, // delta velocity
    pub direction: Vector,
    pub sid: Option<usize>,
    pub new_state: Option<State>,
}
pub struct State {
    pub volume: u32,
    pub kind: Kind,
}
pub fn do_collision(p: &Particle) -> bool {
    !matches!(p.k, Kind::Target | Kind::Anchor)
}
