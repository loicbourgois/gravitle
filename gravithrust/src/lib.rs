use wasm_bindgen::prelude::wasm_bindgen;
mod gravithrust;
mod gravithrust_tick;
mod grid;
mod kind;
mod math;
mod models;
mod particle;
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
use std::ops;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct ModelParticle {
    p: Vector,
    k: Kind,
}

pub struct Ship {
    p: Vector,
    pp: Vector,
    v: Vector, // velocity
    target: Vector,
    td: Vector, // target direction
    orientation: Vector,
    vt: Vector,
    cross: Vector,
    target_pid: usize,
    on_target: u32,
}

pub struct ShipMore {
    pids: Vec<usize>,
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

#[wasm_bindgen]
pub struct ShipModel {
    particles: Vec<ModelParticle>,
    links: Vec<Link>,
}

pub fn kindstr_to_kind(x: &str) -> Kind {
    match x.trim().to_lowercase().as_str() {
        "armor" => Kind::Armor,
        "core" => Kind::Core,
        "booster" => Kind::Booster,
        _ => panic!("invalid kind"),
    }
}

pub fn parse_model(model: &str, diameter: f32) -> ShipModel {
    let model_: &Vec<&str> = &model
        .split('\n')
        .map(str::trim)
        .filter(|line| !line.starts_with('#') && !line.is_empty())
        .collect();
    let start_pair_kinds: &Vec<&str> = &model_
        .iter()
        .filter(|line| line.split(',').count() == 1)
        .copied()
        .collect();
    let model_particles: &Vec<&str> = &model_
        .iter()
        .filter(|line| line.split(',').count() == 4)
        .copied()
        .collect();
    let model_links: &Vec<&str> = &model_
        .iter()
        .filter(|line| line.split(',').count() == 2)
        .copied()
        .collect();
    assert!(start_pair_kinds.len() == 2);
    let mut particles = vec![];
    let mut links = vec![];
    particles.push(ModelParticle {
        p: Vector { x: 0.0, y: 0.0 },
        k: kindstr_to_kind(start_pair_kinds[0]),
    });
    particles.push(ModelParticle {
        p: rotate(
            particles[0].p,
            Vector {
                x: diameter * 1.0,
                y: 0.0,
            },
            4.0 / 6.0,
        ),
        k: kindstr_to_kind(start_pair_kinds[1]),
    });
    for line in model_particles.iter() {
        let terms = line.split(',').collect::<Vec<&str>>();
        let new_particle_id = terms[0].parse::<usize>().expect("invalid particle_id");
        let p1_id = terms[1].parse::<usize>().expect("invalid p1_id");
        let p2_id = terms[2].parse::<usize>().expect("invalid p2_id");
        let kind = kindstr_to_kind(terms[3]);
        assert!(new_particle_id == particles.len(), "bad length");
        particles.push(ModelParticle {
            p: rotate(particles[p1_id].p, particles[p2_id].p, 1.0 / 6.0),
            k: kind,
        });
    }
    for line in model_links.iter() {
        let terms = line.split(',').collect::<Vec<&str>>();
        let pid1 = terms[0].parse::<usize>().expect("invalid pid1");
        let pid2 = terms[1].parse::<usize>().expect("invalid pid2");
        links.push(Link { a: pid1, b: pid2 });
    }
    ShipModel { particles, links }
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
