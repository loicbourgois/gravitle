use rand;
use rand::Rng;
use std::thread;
use std::time;
use wasm_bindgen::prelude::*;
mod math;
use crate::math::collision_response;
use crate::math::wrap_around;
use crate::math::Delta;
use crate::math::Particle;
use crate::math::Vector;
#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub struct Gravithrust {
    particles: Vec<Particle>,
    deltas: Vec<Delta>,
    pub diameter: f32,
}

#[wasm_bindgen]
impl Gravithrust {
    pub fn add_particle(&mut self, p: Vector) {
        self.particles.push(Particle {
            p: p,
            pp: Vector { x: p.x, y: p.y },
            v: Vector { x: 0.0, y: 0.0 },
            m: 1.0,
        });
        self.deltas.push(Delta {
            p: Vector { x: 0.0, y: 0.0 },
            v: Vector { x: 0.0, y: 0.0 },
        });
    }

    pub fn add_ship(&mut self) {
        self.add_particle(Vector { x: 0.5, y: 0.5 });
        self.add_particle(Vector {
            x: 0.5,
            y: 0.5 + self.diameter,
        });
    }

    pub fn new() -> Gravithrust {
        let mut particles = vec![];
        let mut deltas = vec![];
        for i in 0..0 {
            let x = rand::thread_rng().gen::<f32>();
            let y = rand::thread_rng().gen::<f32>();
            let dx = rand::thread_rng().gen::<f32>() * 0.0005 - 0.0005 * 0.5;
            let dy = rand::thread_rng().gen::<f32>() * 0.0005 - 0.0005 * 0.5;
            particles.push(Particle {
                p: Vector { x: x, y: y },
                pp: Vector {
                    x: x - dx,
                    y: y - dy,
                },
                v: Vector { x: dx, y: dy },
                m: 1.0,
            });
            deltas.push(Delta {
                p: Vector { x: 0.0, y: 0.0 },
                v: Vector { x: 0.0, y: 0.0 },
            });
        }

        // for p in vec

        let mut g = Gravithrust {
            particles,
            deltas,
            diameter: 0.03,
        };
        g.add_ship();
        return g;
    }

    pub fn particles_size(&self) -> u32 {
        (self.particles.len() * 7 * 4) as u32
    }

    pub fn particles_count(&self) -> u32 {
        self.particles.len() as u32
    }

    pub fn tick(&mut self) {
        let crdp = 0.01; // collision response delta (position)
        let crdv = 0.9; // collision response delta (velocity)
        let link_strengh = 0.1;
        let linkt_length_ratio = 1.01;
        let diameter = self.diameter;
        let diameter_sqrd = diameter * diameter;
        let booster_acceleration = diameter * 0.01;
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
