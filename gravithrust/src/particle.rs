use crate::error;
use crate::kind::Kind;
use crate::math::delta;
use crate::math::dot;
use crate::math::norm_sqrd;
use crate::math::Vector;
use crate::math::WrapAroundResponse;
use serde::Deserialize;
use serde::Serialize;
use strum_macros::EnumIter;
use wasm_bindgen::prelude::wasm_bindgen;
pub type Particles = Vec<Particle>;
#[wasm_bindgen]
#[derive(
    Serialize, Deserialize, Hash, Copy, Clone, Debug, Eq, EnumIter, Ord, PartialOrd, PartialEq,
)]
#[repr(u32)]
pub enum QuantityKind {
    Invalid      = 0,
    None         = 1,
    Heat         = 2,
    Energy       = 3,
    Matter       = 4,
    Iron         = 5,
    Water        = 6,
    Ice          = 7,
    IronGangue   = 8,
    Coal         = 9,
    IronOre      = 10,
    WaterDroplet = 11,
    Nectar       = 12,
}
#[wasm_bindgen]
#[derive(Clone, Debug, Copy, Default)]
#[repr(C)]
pub struct Quantities {
    pub q1: u32,
    pub q2: u32,
    pub q3: u32,
    pub q4: u32,
    pub q5: u32,
    pub q6: u32,
}
impl Quantities {
    fn by_id(&self, idx: usize) -> u32 {
        match idx {
            0 => self.q1,
            1 => self.q2,
            2 => self.q3,
            3 => self.q4,
            4 => self.q5,
            5 => self.q6,
            _ => panic!("zoop"),
        }
    }

    fn by_id_mut(&mut self, idx: usize) -> &mut u32 {
        match idx {
            0 => &mut self.q1,
            1 => &mut self.q2,
            2 => &mut self.q3,
            3 => &mut self.q4,
            4 => &mut self.q5,
            5 => &mut self.q6,
            _ => panic!("zoop"),
        }
    }
}
#[wasm_bindgen]
#[derive(Clone, Debug)]
#[repr(C)]
pub struct Particle {
    pub qs: Quantities,
    pub p: Vector,
    pub v: Vector,
    pub pp: Vector,
    pub direction: Vector,
    pub m: f32,
    pub k: Kind,
    pub a: u32, // activated, usefull for boosters
    pub live: u32,
    pub grid_id: usize,
    pub idx: usize,
}
impl Particle {
    pub fn capacity(&self, qk: QuantityKind) -> u32 {
        match (self.k, qk) {
            (Kind::IronAsteroid, QuantityKind::IronOre) => 2,
            (Kind::CoalAsteroid, QuantityKind::Coal) => 2,
            (Kind::IceAsteroid, QuantityKind::Ice) => 1,
            (Kind::CoalDepot, QuantityKind::Coal) => 100,
            (Kind::CoalCargo, QuantityKind::Coal) => 10,
            (Kind::CoalCollector, QuantityKind::Coal) => 1,
            (Kind::IronOreCollector, QuantityKind::IronOre) => 1,
            (Kind::IronOreDepot, QuantityKind::IronOre) => 100,
            (Kind::IronOreCargo, QuantityKind::IronOre) => 10,
            (Kind::IronFurnace, QuantityKind::Coal) => 1,
            (Kind::IronFurnace, QuantityKind::IronGangue) => 1,
            (Kind::IronFurnace, QuantityKind::Heat) => 1,
            (Kind::IronFurnace, QuantityKind::IronOre) => 1,
            (Kind::IronFurnace, QuantityKind::Iron) => 1,

            (Kind::Booster, QuantityKind::Energy) => 1000,
            (Kind::Core, QuantityKind::Energy) => 100_000,
            (Kind::Battery, QuantityKind::Energy) => 100_000,
            (Kind::EnergyCollector, QuantityKind::Energy) => 1_000_000,
            (Kind::EnergyCargo, QuantityKind::Energy) => 1_000_000,
            (Kind::EnergyDepot, QuantityKind::Energy) => 1_000_000,
            _ => 0,
        }
    }

    pub fn remaining_capacity(&self, qk: QuantityKind) -> u32 {
        self.capacity(qk) - self.quantity(qk)
    }

    pub fn quantity(&self, qk: QuantityKind) -> u32 {
        match self.qks().iter().position(|&x| x == qk) {
            Some(idx) => self.qs.by_id(idx),
            None => {
                error(&format!(
                    "quantity {:?} - {:?} - {:?}",
                    self.k,
                    qk,
                    self.qks()
                ));
                panic!();
            }
        }
    }

    pub fn add_quantity(&mut self, qk: QuantityKind, q: u32) {
        match self.qks().iter().position(|&x| x == qk) {
            Some(idx) => *self.qs.by_id_mut(idx) += q,
            None => {
                error(&format!("add_quantity {:?}: {:?}", self.k, qk));
                panic!();
            }
        }
    }

    pub fn remove_quantity(&mut self, qk: QuantityKind, q: u32) {
        match self.qks().iter().position(|&x| x == qk) {
            Some(idx) => *self.qs.by_id_mut(idx) -= q,
            None => {
                error(&format!("remove_quantity {:?}: {:?}", self.k, qk));
                panic!();
            }
        }
    }
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
            qs: Quantities::default(),
            live: 0,
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
