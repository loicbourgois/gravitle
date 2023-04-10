use wasm_bindgen::prelude::wasm_bindgen;
mod blueprint;
mod gravithrust;
mod gravithrust_tick;
mod grid;
mod kind;
mod math;
mod particle;
mod ship;
mod test;
use crate::kind::Kind;
use crate::math::collision_response;
use crate::math::cross;
use crate::math::normalize;
use crate::math::normalize_2;
use crate::math::rotate;
use crate::math::wrap_around;
use crate::math::Delta;
use crate::math::Vector;
use crate::particle::Particle;
use crate::ship::ShipMore;
use std::ops;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Link {
    a: usize,
    b: usize,
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct LinkJS {
    #[allow(dead_code)]
    ak: Kind,
    #[allow(dead_code)]
    bk: Kind,
    p: Vector,
}

pub fn kindstr_to_kind(x: &str) -> Kind {
    match x.trim().to_lowercase().as_str() {
        "armor" => Kind::Armor,
        "core" => Kind::Core,
        "booster" => Kind::Booster,
        "ray" => Kind::Ray,
        "cargo" => Kind::Cargo,
        "sun" => Kind::Sun,
        _ => panic!("invalid kind"),
    }
}

impl ops::Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::Div<f32> for Vector {
    type Output = Vector;
    fn div(self, other: f32) -> Vector {
        Vector {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl ops::Mul<f32> for Vector {
    type Output = Vector;
    fn mul(self, other: f32) -> Vector {
        Vector {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl ops::Sub<Vector> for Vector {
    type Output = Vector;
    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

pub fn ship_position(particles: &[Particle], s: &ShipMore) -> Vector {
    let p0 = &particles[s.pids[0]];
    let mut p = p0.pp;
    for i in 1..s.pids.len() {
        let pid = s.pids[i];
        let p1 = &particles[pid];
        let uu = wrap_around(p0.pp, p1.pp).d;
        p = p + uu / s.pids.len() as f32;
    }
    p
}

pub fn ship_orientation(particles: &[Particle], s: &ShipMore) -> Vector {
    let p0 = &particles[s.pids[0]];
    let p1 = &particles[s.pids[1]];
    let p2 = &particles[s.pids[2]];
    let p12 = p1.p + wrap_around(p1.p, p2.p).d / 2.0;
    wrap_around(p12, p0.p).d
}
