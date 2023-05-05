use crate::kind::Kind;
use crate::math::Vector;
use wasm_bindgen::prelude::wasm_bindgen;
pub type Particles = Vec<Particle>;
#[wasm_bindgen]
#[derive(Clone, Debug)]
#[repr(C)]
pub struct Particle {
    pub p: Vector,
    pub v: Vector,
    pub pp: Vector,
    pub direction: Vector,
    pub m: f32,
    pub k: Kind,
    pub a: u32, // activated, usefull for boosters
    pub quantity: u32,
    pub live: u32,
    pub grid_id: usize,
    pub idx: usize,
    pub packer: f32,
}
impl Default for Particle {
    fn default() -> Self {
        Particle {
            p: Vector::default(),
            pp: Vector::default(),
            v: Vector::default(),
            m: 1.0,
            k: Kind::Sun,
            direction: Vector::default(),
            a: 0,
            idx: 0,
            grid_id: 0,
            quantity: 0,
            live: 0,
            packer: -0.12,
        }
    }
}
#[derive(Default)]
pub struct ParticleInternal {
    pub dp: Vector, // delta position
    pub dv: Vector, // delta velocity
    pub direction: Vector,
    pub sid: Option<usize>,
    pub new_state: Option<State>,
}
pub struct State {
    pub quantity: u32,
    pub kind: Kind,
    pub live: u32,
}
pub fn do_collision(p: &Particle) -> bool {
    !matches!(p.k, Kind::Target | Kind::Anchor)
}
