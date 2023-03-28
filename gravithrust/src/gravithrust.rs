use crate::cross;
use crate::gravithrust_tick::add_particle;
use crate::gravithrust_tick::add_particles;
use crate::gravithrust_tick::compute_collision_responses;
use crate::gravithrust_tick::compute_link_responses;
use crate::gravithrust_tick::update_particles;
use crate::grid::Grid;
use crate::kind::Kind;
use crate::log;
use crate::models::MODEL_1;
use crate::normalize_2;
use crate::parse_model;
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
use rand::Rng;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Gravithrust {
    particles: Vec<Particle>,
    ships: Vec<Ship>,
    links: Vec<Link>,
    deltas: Vec<Delta>,
    ships_more: Vec<ShipMore>,
    pub diameter: f32,
    grid: Grid,
    pub points: u32,
    pub step: u32,
    pub sub_steps: usize,
    pub turn_speed_a: f32,
    pub turn_speed_b: f32,
}

#[wasm_bindgen]
impl Gravithrust {
    pub fn new(
        diameter: f32,
        sub_steps: usize,
        turn_speed_a: f32,
        turn_speed_b: f32,
    ) -> Gravithrust {
        let grid_side = 128;
        assert!((diameter * grid_side as f32) <= 1.0);
        let mut g = Gravithrust {
            particles: vec![],
            links: vec![],
            deltas: vec![],
            ships: vec![],
            ships_more: vec![],
            diameter: diameter,
            grid: Grid::new(grid_side),
            points: 0,
            step: 0,
            sub_steps: sub_steps,
            turn_speed_a: turn_speed_a,
            turn_speed_b: turn_speed_b,
        };
        for _ in 0..1 {
            g.add_particle(Vector { x: 0.35, y: 0.5 }, Kind::Metal, None);
            g.add_particle(Vector { x: 0.65, y: 0.5 }, Kind::Depot, None);
        }
        for _ in 0..10 {
            g.add_ship(
                &parse_model(MODEL_1, g.diameter),
                Vector {
                    x: rand::thread_rng().gen::<f32>(),
                    y: rand::thread_rng().gen::<f32>(),
                },
            );
        }
        return g;
    }

    pub fn add_particle(&mut self, p: Vector, k: Kind, sid: Option<usize>) {
        add_particle(&mut self.particles, &mut self.deltas, p, k, sid);
    }

    pub fn add_ship(&mut self, ship_model: &ShipModel, position: Vector) {
        let pid_start = self.particles.len();
        let sid = Some(self.ships.len());
        let ship = Ship {
            target: Vector {
                x: rand::thread_rng().gen::<f32>(),
                y: rand::thread_rng().gen::<f32>(),
            },
            p: Vector { x: 0.0, y: 0.0 },
            pp: Vector { x: 0.0, y: 0.0 },
            v: Vector { x: 0.0, y: 0.0 },
            td: Vector { x: 0.0, y: 0.0 },
            orientation: Vector { x: 0.0, y: 0.0 },
            vt: Vector { x: 0.0, y: 0.0 },
            cross: Vector { x: 0.0, y: 0.0 },
            on_target: 0,
            target_pid: 0,
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

    pub fn particles_size(&self) -> u32 {
        (self.particles.len() * self.particle_size_()) as u32
    }

    pub fn particle_size(&self) -> u32 {
        self.particle_size_() as u32
    }

    pub fn particle_size_(&self) -> usize {
        13 * 4
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
        18 * 4
    }

    pub fn ships_count(&self) -> u32 {
        self.ships.len() as u32
    }

    pub fn ships(&self) -> *const Ship {
        self.ships.as_ptr()
    }
}

#[wasm_bindgen]
impl Gravithrust {
    pub fn ticks(&mut self) {
        for _ in 0..self.sub_steps {
            self.tick()
        }
    }

    fn tick(&mut self) {
        add_particles(self.diameter, &mut self.particles, &mut self.deltas);
        self.grid.update_01();
        self.grid.update_02(&mut self.particles);
        compute_collision_responses(
            self.diameter,
            &mut self.particles,
            &mut self.deltas,
            &self.grid,
            &mut self.ships,
        );
        compute_link_responses(
            self.diameter,
            &mut self.particles,
            &mut self.deltas,
            &mut self.links,
        );
        update_particles(self.diameter, &mut self.particles, &mut self.deltas);
        for (sid, s) in self.ships_more.iter_mut().enumerate() {
            let pid0 = s.pids[0];
            let pid_left = pid0 + 10;
            let pid_right = pid0 + 14;
            let mut ship = &mut self.ships[sid];
            if ship.on_target >= 1 {
                self.points += 1;
                log("on target");
                if ship.target_pid == 0 {
                    ship.target_pid = 1
                } else {
                    ship.target_pid = 0
                }
            }
            ship.target = self.particles[ship.target_pid].p;
            ship.on_target = 0;
            ship.pp = ship.p;
            ship.p = ship_position(&self.particles, &s);
            // velocity
            ship.v = wrap_around(ship.pp, ship.p).d;
            // target delta
            ship.td = wrap_around(ship.p, ship.target).d;
            let previous_orientation = ship.orientation;
            // orientation is where the ship is facing
            ship.orientation = ship_orientation(&self.particles, &s);
            ship.vt = normalize_2(normalize_2(ship.td) + normalize_2(ship.v));
            ship.cross =
                normalize_2(normalize_2(ship.orientation) * 1.0 + normalize_2(ship.v) * 0.5);
            let cross_2_ = cross(ship.cross, ship.td);
            let cross_3_ = cross(ship.orientation, previous_orientation);
            if cross_2_ < 0.0 && cross_3_ < self.turn_speed_a {
                self.particles[pid_left].a = 1
            }
            if cross_2_ > 0.0 && cross_3_ > -self.turn_speed_a {
                self.particles[pid_right].a = 1
            }
            if cross_3_ > self.turn_speed_b {
                self.particles[pid_right].a = 1;
                self.particles[pid_left].a = 0;
                // log("aa");
            } else if cross_3_ < -self.turn_speed_b {
                self.particles[pid_left].a = 1;
                self.particles[pid_right].a = 0;
                // log("bb");
            }
        }
        self.step += 1;
    }
}
