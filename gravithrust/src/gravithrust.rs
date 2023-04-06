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
    pub max_rotation_speed: f32,
    pub max_speed_at_target: f32,
    pub forward_max_speed: f32,
    pub forward_max_angle: f32,
    pub slow_down_max_angle: f32,
    pub slow_down_max_speed_to_target_ratio: f32,
}

#[wasm_bindgen]
impl Gravithrust {
    pub fn new(
        diameter: f32,
        sub_steps: usize,
        max_rotation_speed: f32,
        grid_side: u32,
        max_speed_at_target: f32,
        forward_max_speed: f32,
        forward_max_angle: f32,
        slow_down_max_angle: f32,
        slow_down_max_speed_to_target_ratio: f32,
    ) -> Gravithrust {
        assert!((diameter * grid_side as f32) <= 1.0);
        let mut g = Gravithrust {
            particles: vec![],
            links: vec![],
            deltas: vec![],
            ships: vec![],
            ships_more: vec![],
            diameter: diameter,
            grid: Grid::new(grid_side as usize),
            points: 0,
            step: 0,
            sub_steps: sub_steps,
            max_rotation_speed: max_rotation_speed,
            max_speed_at_target: max_speed_at_target,
            forward_max_speed: forward_max_speed,
            forward_max_angle: forward_max_angle,
            slow_down_max_angle: slow_down_max_angle,
            slow_down_max_speed_to_target_ratio: slow_down_max_speed_to_target_ratio,
        };
        for _ in 0..1 {
            g.add_particle(Vector { x: 0.35, y: 0.35 }, Kind::Target, None);
            g.add_particle(Vector { x: 0.35, y: 0.65 }, Kind::Target, None);
            g.add_particle(Vector { x: 0.65, y: 0.65 }, Kind::Target, None);
            g.add_particle(Vector { x: 0.65, y: 0.35 }, Kind::Target, None);
        }
        for _ in 0..20 {
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
            target_pid: self.ships.len() % 4,
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
            let pid_up_right = pid0 + 3;
            let pid_up_left = pid0 + 6;
            let mut ship = &mut self.ships[sid];
            ship.pp = ship.p;
            ship.p = ship_position(&self.particles, &s);
            // velocity
            let wa = wrap_around(ship.pp, ship.p);
            let speed = wa.d_sqrd.sqrt();
            if ship.on_target >= 1 && speed.abs() < self.max_speed_at_target {
                self.points += 1;
                log("on target");
                if ship.target_pid == 0 {
                    ship.target_pid = 1
                } else if ship.target_pid == 1 {
                    ship.target_pid = 2
                } else if ship.target_pid == 2 {
                    ship.target_pid = 3
                } else {
                    ship.target_pid = 0
                }
            }
            ship.target = self.particles[ship.target_pid].p;
            ship.on_target = 0;
            ship.v = wa.d;
            let target_delta_wa = wrap_around(ship.p, ship.target);
            // target delta
            ship.td = target_delta_wa.d;
            let old_target_delta = wrap_around(ship.pp, ship.target);
            let distance_to_target = target_delta_wa.d_sqrd.sqrt();
            let speed_toward_target = old_target_delta.d_sqrd.sqrt() - distance_to_target;
            let previous_orientation = ship.orientation;
            // orientation is where the ship is facing
            ship.orientation = ship_orientation(&self.particles, &s);
            ship.vt = normalize_2(normalize_2(ship.td) + normalize_2(ship.v));
            ship.cross =
                normalize_2(normalize_2(ship.orientation) * 1.0 + normalize_2(ship.v) * 0.5);
            let orientation_angle = cross(normalize_2(ship.orientation), normalize_2(ship.td));
            let orientation_angle_corrected = cross(normalize_2(ship.cross), normalize_2(ship.td));
            let rotation_speed = cross(ship.orientation, previous_orientation);
            let velocity_vs_target_angle = cross(normalize_2(ship.v), normalize_2(ship.td));
            let mut action = "-";
            self.particles[pid_left].a = 0;
            self.particles[pid_right].a = 0;
            self.particles[pid_up_right].a = 0;
            self.particles[pid_up_left].a = 0;
            if speed < self.forward_max_speed
                && orientation_angle_corrected.abs() < self.forward_max_angle
            {
                action = "forward";
                self.particles[pid_left].a = 1;
                self.particles[pid_right].a = 1;
                self.particles[pid_up_right].a = 0;
                self.particles[pid_up_left].a = 0;
            }
            if orientation_angle_corrected > 0.0 && rotation_speed > -self.max_rotation_speed {
                action = "turn left";
                self.particles[pid_left].a = 0;
                self.particles[pid_right].a = 1;
                self.particles[pid_up_right].a = 0;
                self.particles[pid_up_left].a = 1;
            } else if orientation_angle_corrected < 0.0 && rotation_speed < self.max_rotation_speed
            {
                action = "turn right";
                self.particles[pid_left].a = 1;
                self.particles[pid_right].a = 0;
                self.particles[pid_up_right].a = 1;
                self.particles[pid_up_left].a = 0;
            }
            if orientation_angle_corrected.abs() < self.slow_down_max_angle
                && speed_toward_target
                    > (self.max_speed_at_target * 0.75)
                        .max(distance_to_target * self.slow_down_max_speed_to_target_ratio)
            {
                action = "slow down";
                self.particles[pid_left].a = 0;
                self.particles[pid_right].a = 0;
                self.particles[pid_up_right].a = 1;
                self.particles[pid_up_left].a = 1;
            }
            // log(&format!(
            //     "distance_to_target: {}\norientation_angle: {}\norientation_angle_corrected: {}\nrotation_speed: {}\nspeed: {}\nvelocity_vs_target_angle: {}\nspeed_toward_target: {}\naction:{}",
            //     distance_to_target, orientation_angle,
            //     orientation_angle_corrected, rotation_speed, speed,
            //     velocity_vs_target_angle, speed_toward_target, action
            // ));
        }
        self.step += 1;
    }
}
