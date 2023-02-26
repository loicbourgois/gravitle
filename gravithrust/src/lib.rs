use rand;
use rand::Rng;
use std::thread;
use std::time;
use wasm_bindgen::prelude::*;
mod math;
mod models;
use crate::math::collision_response;
use crate::math::rotate;
use crate::math::wrap_around;
use crate::math::Delta;
use crate::math::Particle;
use crate::math::Vector;
use crate::models::MODEL_1;
use std::ops::Add;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum Kind {
    armor = 1,
    core = 2,
    booster = 3,
}

pub struct ModelParticle {
    p: Vector,
    k: Kind,
}

#[wasm_bindgen]
pub struct Gravithrust {
    particles: Vec<Particle>,
    deltas: Vec<Delta>,
    pub diameter: f32,
}

#[wasm_bindgen]
pub struct Ship {
    particles: Vec<ModelParticle>,
    links: Vec<usize>,
}

pub fn kindstr_to_kind(x: &str) -> Kind {
    match x.trim().to_lowercase().as_str() {
        "armor" => Kind::armor,
        "core" => Kind::armor,
        "booster" => Kind::armor,
        _ => panic!("invalid kind"),
    }
}

pub fn parse_model(model: &str, diameter: f32) -> Ship {
    let model_: &Vec<&str> = &model
        .split("\n")
        .map(|line| line.trim())
        .filter(|line| !line.starts_with("#") && line.len() > 0)
        .collect();
    let start_pair_kinds: &Vec<&str> = &model_
        .iter()
        .filter(|line| line.split(",").collect::<Vec<&str>>().len() == 1)
        .map(|x| *x)
        .collect();
    let model_particles: &Vec<&str> = &model_
        .iter()
        .filter(|line| line.split(",").collect::<Vec<&str>>().len() == 4)
        .map(|x| *x)
        .collect();
    assert!(start_pair_kinds.len() == 2);
    // log(start_pair_kinds[0]);
    // log(start_pair_kinds[1]);
    let mut particles = vec![];
    particles.push(ModelParticle {
        p: Vector { x: 0.0, y: 0.0 },
        k: kindstr_to_kind(start_pair_kinds[0]),
    });
    particles.push(ModelParticle {
        p: rotate(
            particles[0].p,
            Vector {
                x: diameter,
                y: 0.0,
            },
            4.0 / 6.0,
        ),
        k: kindstr_to_kind(start_pair_kinds[1]),
    });
    for line in model_particles.iter() {
        let terms = line.split(",").collect::<Vec<&str>>();
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
    return Ship {
        particles: particles,
        links: vec![],
    };
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[wasm_bindgen]
impl Gravithrust {
    pub fn add_particle(&mut self, p: Vector, k: Kind) {
        self.particles.push(Particle {
            p: p,
            pp: Vector { x: p.x, y: p.y },
            v: Vector { x: 0.0, y: 0.0 },
            m: 1.0,
            k,
        });
        self.deltas.push(Delta {
            p: Vector { x: 0.0, y: 0.0 },
            v: Vector { x: 0.0, y: 0.0 },
        });
    }

    pub fn add_ship(&mut self, ship: &Ship, position: Vector) {
        for p in &ship.particles {
            self.add_particle(p.p + position, p.k);
        }
    }

    pub fn new() -> Gravithrust {
        // for i in 0..0 {
        //     let x = rand::thread_rng().gen::<f32>();
        //     let y = rand::thread_rng().gen::<f32>();
        //     let dx = rand::thread_rng().gen::<f32>() * 0.0005 - 0.0005 * 0.5;
        //     let dy = rand::thread_rng().gen::<f32>() * 0.0005 - 0.0005 * 0.5;
        //     particles.push(Particle {
        //         p: Vector { x: x, y: y },
        //         pp: Vector {
        //             x: x - dx,
        //             y: y - dy,
        //         },
        //         v: Vector { x: dx, y: dy },
        //         m: 1.0,
        //     });
        //     deltas.push(Delta {
        //         p: Vector { x: 0.0, y: 0.0 },
        //         v: Vector { x: 0.0, y: 0.0 },
        //     });
        // }
        let mut g = Gravithrust {
            particles: vec![],
            deltas: vec![],
            diameter: 0.03,
        };
        g.add_ship(&parse_model(MODEL_1, g.diameter), Vector { x: 0.5, y: 0.5 });
        return g;
    }

    pub fn particles_size(&self) -> u32 {
        (self.particles.len() * self.particle_size_()) as u32
    }

    pub fn particle_size(&self) -> u32 {
        self.particle_size_() as u32
    }

    pub fn particle_size_(&self) -> usize {
        8 * 4
    }

    pub fn particles_count(&self) -> u32 {
        self.particles.len() as u32
    }

    pub fn tick(&mut self) {
        let crdp = 0.01; // collision response delta (position)
        let crdv = 0.90; // collision response delta (velocity)
        let link_strengh = 0.1;
        let linkt_length_ratio = 1.01;
        let diameter = self.diameter;
        let diameter_sqrd = diameter * diameter;
        // let booster_acceleration = diameter * 0.01;
        for (i1, p1) in self.particles.iter().enumerate() {
            for (i2, p2) in self.particles.iter().enumerate() {
                if i1 < i2 {
                    let wa = wrap_around(p1.p, p2.p);
                    if wa.d_sqrd < diameter_sqrd {
                        let cr = collision_response(&wa, p1, p2);
                        if !cr.x.is_nan() && !cr.y.is_nan() {
                            {
                                let d1 = &mut self.deltas[i1];
                                d1.v.x += cr.x * crdv;
                                d1.v.y += cr.y * crdv;
                                d1.p.x -= wa.d.x * crdp;
                                d1.p.y -= wa.d.y * crdp;
                            }
                            {
                                let d2 = &mut self.deltas[i2];
                                d2.v.x -= cr.x * crdv;
                                d2.v.y -= cr.y * crdv;
                                d2.p.x += wa.d.x * crdp;
                                d2.p.y += wa.d.y * crdp;
                            }
                        }
                    }
                }
            }
        }
        for (i1, p1) in self.particles.iter_mut().enumerate() {
            let mut d1 = &mut self.deltas[i1];
            p1.v.x += d1.v.x;
            p1.v.y += d1.v.y;
            p1.p.x += d1.p.x;
            p1.p.y += d1.p.y;
            d1.p.x = 0.0;
            d1.p.y = 0.0;
            d1.v.x = 0.0;
            d1.v.y = 0.0;
            p1.v.x = p1.v.x.max(-diameter * 0.5);
            p1.v.x = p1.v.x.min(diameter * 0.5);
            p1.v.y = p1.v.y.max(-diameter * 0.5);
            p1.v.y = p1.v.y.min(diameter * 0.5);
            p1.p.x = (10.0 + p1.p.x + p1.v.x) % 1.0;
            p1.p.y = (10.0 + p1.p.y + p1.v.y) % 1.0;
            p1.pp.x = p1.p.x - p1.v.x;
            p1.pp.y = p1.p.y - p1.v.y;
        }
    }

    pub fn particles(&self) -> *const Particle {
        self.particles.as_ptr()
    }
}
