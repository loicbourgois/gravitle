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
    pub grid_id: usize,
    pub idx: usize,
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
