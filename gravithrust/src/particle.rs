use crate::kind::Kind;
use crate::math::delta;
use crate::math::dot;
use crate::math::norm_sqrd;
use crate::math::Vector;
use crate::math::WrapAroundResponse;
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
    // pub q1: u32,
    // pub q2: u32,
    // pub qk1: QuantityKind,
    // pub qk2: QuantityKind,
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
pub fn collision_response(wa: &WrapAroundResponse, p1: &Particle, p2: &Particle) -> Vector {
    // https://en.wikipedia.org/wiki/Elastic_collision#Two-dimensional_collision_with_two_moving_objects
    let delta_velocity = delta(p1.v, p2.v);
    let delta_position = wa.d;
    let mass_factor = 2.0 * p2.m / (p2.m + p1.m);
    let dot_vp = dot(delta_velocity, delta_position);
    let n_sqrd = norm_sqrd(delta_position);
    let factor = mass_factor * dot_vp / n_sqrd;
    Vector {
        x: delta_position.x * factor,
        y: delta_position.y * factor,
    }
}
