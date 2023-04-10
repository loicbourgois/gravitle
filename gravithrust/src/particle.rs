use crate::Kind;
use crate::Vector;
use wasm_bindgen::prelude::wasm_bindgen;

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
    pub grid_id: usize,
    pub idx: usize,
}

pub fn is_static(p: &Particle) -> bool {
    matches!(p.k, Kind::Sun | Kind::Metal | Kind::Depot | Kind::Anchor)
}

pub fn do_collision(p: &Particle) -> bool {
    !matches!(p.k, Kind::Target | Kind::Anchor)
}
