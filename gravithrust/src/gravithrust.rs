// use crate::collision_response;
use crate::cross;
use rand::Rng;
// use crate::gravithrust::Gravithrust;
use crate::gravithrust_tick::compute_collision_responses;
use crate::gravithrust_tick::compute_link_responses;
use crate::gravithrust_tick::update_particles;
use crate::kind::Kind;
use crate::log;
use crate::models::MODEL_1;
// use crate::normalize;
use crate::normalize_2;
use crate::parse_model;
// use crate::particle;
use crate::ship_orientation;
use crate::ship_position;
use crate::wrap_around;
use crate::Delta;
use crate::Link;
use crate::Particle;
use crate::Ship;
use crate::ShipModel;
use crate::ShipMore;
use crate::Vector;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Gravithrust {
    particles: Vec<Particle>,
    ships: Vec<Ship>,
    links: Vec<Link>,
    deltas: Vec<Delta>,
    ships_more: Vec<ShipMore>,
    pub diameter: f32,
}

#[wasm_bindgen]
impl Gravithrust {
    pub fn add_particle(&mut self, p: Vector, k: Kind, sid: Option<usize>) {
        self.particles.push(Particle {
            p: p,
            pp: Vector { x: p.x, y: p.y },
            v: Vector { x: 0.0, y: 0.0 },
            m: 1.0,
            k,
            direction: Vector { x: 0.0, y: 0.0 },
            a: 0,
        });
        self.deltas.push(Delta {
            p: Vector { x: 0.0, y: 0.0 },
            v: Vector { x: 0.0, y: 0.0 },
            direction: Vector { x: 0.0, y: 0.0 },
            sid: sid,
        });
    }

    pub fn add_ship(&mut self, ship_model: &ShipModel, position: Vector) {
        let pid_start = self.particles.len();
        let sid = Some(self.ships.len());
        let ship = Ship {
            target: Vector { x: 0.5, y: 0.5 },
            p: Vector { x: 0.0, y: 0.0 },
            pp: Vector { x: 0.0, y: 0.0 },
            v: Vector { x: 0.0, y: 0.0 },
            td: Vector { x: 0.0, y: 0.0 },
            orientation: Vector { x: 0.0, y: 0.0 },
            vt: Vector { x: 0.0, y: 0.0 },
        };
        let mut ship_more = ShipMore { pids: vec![] };
        for p in &ship_model.particles {
            ship_more.pids.push(self.particles.len());
            self.add_particle(p.p + position, p.k, sid);
        }
        for l in &ship_model.links {
            self.links.push(Link {
                a: l.a + pid_start,
                b: l.b + pid_start,
            })
        }
        self.ships.push(ship);
        self.ships[sid.unwrap()].p = ship_position(&self.particles, &ship_more);
        self.ships_more.push(ship_more);
        self.ships[sid.unwrap()].pp = self.ships[sid.unwrap()].p;
    }

    pub fn new() -> Gravithrust {
        let mut g = Gravithrust {
            particles: vec![],
            links: vec![],
            deltas: vec![],
            ships: vec![],
            ships_more: vec![],
            diameter: 0.005,
        };
        g.add_particle(Vector { x: 0.5, y: 0.5 }, Kind::Sun, None);
        if false {
            g.add_ship(
                &parse_model(MODEL_1, g.diameter),
                Vector { x: 0.75, y: 0.5 },
            );
            g.add_ship(
                &parse_model(MODEL_1, g.diameter),
                Vector { x: 0.25, y: 0.5 },
            );
            g.add_ship(&parse_model(MODEL_1, g.diameter), Vector { x: 0.5, y: 0.5 });
            g.add_ship(&parse_model(MODEL_1, g.diameter), Vector { x: 0.0, y: 0.5 });
        }
        return g;
    }

    pub fn particles_size(&self) -> u32 {
        (self.particles.len() * self.particle_size_()) as u32
    }

    pub fn particle_size(&self) -> u32 {
        self.particle_size_() as u32
    }

    pub fn particle_size_(&self) -> usize {
        11 * 4
    }

    pub fn particles_count(&self) -> u32 {
        self.particles.len() as u32
    }

    pub fn particles(&self) -> *const Particle {
        self.particles.as_ptr()
    }

    pub fn ships_size(&self) -> u32 {
        (self.ships.len() * self.ship_size_()) as u32
    }

    pub fn ship_size(&self) -> u32 {
        self.ship_size_() as u32
    }

    pub fn ship_size_(&self) -> usize {
        14 * 4
    }

    pub fn ships_count(&self) -> u32 {
        self.ships.len() as u32
    }

    pub fn ships(&self) -> *const Ship {
        self.ships.as_ptr()
    }
}

struct ParticleModel {
    p: Vector,
    k: Kind,
    sid: Option<usize>,
}

#[wasm_bindgen]
impl Gravithrust {
    pub fn tick(&mut self) {
        let mut particles_to_add = vec![];
        for p1 in &self.particles {
            if p1.k == Kind::Sun && rand::thread_rng().gen::<f32>() < 0.01 {
                particles_to_add.push(ParticleModel {
                    p: Vector {
                        x: p1.p.x + rand::thread_rng().gen::<f32>() * self.diameter
                            - self.diameter * 0.5,
                        y: p1.p.y + rand::thread_rng().gen::<f32>() * self.diameter
                            - self.diameter * 0.5,
                    },
                    k: Kind::Armor,
                    sid: None,
                })
            }
        }

        for x in &particles_to_add {
            self.add_particle(x.p, x.k, x.sid);
        }

        compute_collision_responses(self.diameter, &mut self.particles, &mut self.deltas);
        compute_link_responses(
            self.diameter,
            &mut self.particles,
            &mut self.deltas,
            &mut self.links,
        );
        update_particles(self.diameter, &mut self.particles, &mut self.deltas);
        for (sid, s) in self.ships_more.iter_mut().enumerate() {
            let pid0 = s.pids[0];
            let mut ship = &mut self.ships[sid];
            ship.target = self.particles[0].p;
            ship.pp = ship.p;
            ship.p = ship_position(&self.particles, &s);
            ship.v = wrap_around(ship.pp, ship.p).d;
            ship.td = wrap_around(ship.p, ship.target).d;
            let previous_orientation = ship.orientation;
            ship.orientation = ship_orientation(&self.particles, &s);
            ship.vt = normalize_2(normalize_2(ship.td) + normalize_2(ship.v));
            // let cross__ = cross(ship.orientation, ship.vt);
            let cross_2_ = cross(
                (normalize_2(ship.v) + normalize_2(ship.orientation)) / 2.0,
                ship.td,
            );
            let cross_3_ = cross(ship.orientation, previous_orientation);
            let turn_speed = 0.000000001;
            if cross_2_ < 0.0 && cross_3_ < turn_speed {
                self.particles[pid0 + 10].a = 1
            }
            if cross_2_ > 0.0 && cross_3_ > -turn_speed {
                self.particles[pid0 + 14].a = 1
            }
            // else {
            //     self.particles[14].a = 1
            // }
            // if cross_2_ > 0.0 {
            //     self.particles[14].a = 1
            // }
            // else {
            //     self.particles[14].a = 1
            // }
            // log(&format!("{}", a));
            // let aa = dot(self.ships[sid].v, self.ships[sid].td);
            // log(&format!("{} ", a));
            // for pid in &s.pids {
            //     let mut p1 = &mut self.particles[*pid];
            //     if p1.k == Kind::booster {
            //         // p1.a = 1;
            //     }
            // }
        }
    }
}
