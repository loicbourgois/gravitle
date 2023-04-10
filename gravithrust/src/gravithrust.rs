use crate::blueprint::load_raw_blueprint;
use crate::blueprint::Blueprint;
use crate::blueprint::RawBlueprint;
use crate::cross;
use crate::gravithrust_tick::add_particle;
use crate::gravithrust_tick::add_particles;
use crate::gravithrust_tick::compute_collision_responses;
use crate::gravithrust_tick::compute_link_responses;
use crate::gravithrust_tick::update_particles;
use crate::grid::Grid;
use crate::kind::Kind;
use crate::log;
use crate::math::angle;
use crate::math::radians;
use crate::normalize_2;
use crate::ship::Ship;
use crate::ship::ShipControl;
use crate::ship::ShipMore;
use crate::ship_orientation;
use crate::ship_position;
use crate::wrap_around;
use crate::Delta;
use crate::Link;
use crate::LinkJS;
use crate::Particle;
use crate::Vector;
use rand::Rng;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Gravithrust {
    particles: Vec<Particle>,
    ships: Vec<Ship>,
    links: Vec<Link>,
    links_js: Vec<LinkJS>,
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
    pub booster_acceleration: f32,
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
        booster_acceleration: f32,
    ) -> Gravithrust {
        assert!((diameter * grid_side as f32) <= 1.0);
        let mut g = Gravithrust {
            particles: vec![],
            links: vec![],
            deltas: vec![],
            ships: vec![],
            ships_more: vec![],
            links_js: vec![],
            diameter,
            grid: Grid::new(grid_side as usize),
            points: 0,
            step: 0,
            sub_steps,
            max_rotation_speed,
            max_speed_at_target,
            forward_max_speed,
            forward_max_angle,
            slow_down_max_angle,
            slow_down_max_speed_to_target_ratio,
            booster_acceleration,
        };
        g.add_particle(Vector { x: 0.35, y: 0.35 }, Kind::Target, None);
        g.add_particle(Vector { x: 0.35, y: 0.65 }, Kind::Target, None);
        g.add_particle(Vector { x: 0.65, y: 0.65 }, Kind::Target, None);
        g.add_particle(Vector { x: 0.65, y: 0.35 }, Kind::Target, None);
        g.add_particle(Vector { x: 0.5, y: 0.5 }, Kind::Sun, None);
        g.add_particle(Vector { x: 0.55, y: 0.5 }, Kind::Anchor, None);
        g
    }

    pub fn add_ship(&mut self, yml_blueprint: &str, x: f32, y: f32) {
        let raw_blueprint: RawBlueprint = serde_yaml::from_str(yml_blueprint).unwrap();
        let blueprint = load_raw_blueprint(&raw_blueprint, self.diameter);
        self.add_ship_2(&blueprint, Vector { x, y });
    }

    pub fn add_particle(&mut self, p: Vector, k: Kind, sid: Option<usize>) {
        add_particle(&mut self.particles, &mut self.deltas, p, k, sid);
    }

    pub fn add_ship_2(&mut self, ship_model: &Blueprint, position: Vector) {
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
            anchor_pid: match self.ships.len() {
                0 => Some(5),
                _ => None,
            },
        };
        let mut ship_more = ShipMore {
            pids: vec![],
            ship_control: ShipControl {
                left: ship_model.left.clone(),
                right: ship_model.right.clone(),
                slow: ship_model.slow.clone(),
                forward: ship_model.forward.clone(),
                translate_left: ship_model.translate_left.clone(),
                translate_right: ship_model.translate_right.clone(),
            },
        };
        for p in &ship_model.particles {
            ship_more.pids.push(self.particles.len());
            self.add_particle(p.p + position, p.k, sid);
        }
        for l in &ship_model.links {
            self.links.push(Link {
                a: l.a + pid_start,
                b: l.b + pid_start,
            });
            let pa = &self.particles[l.a + pid_start];
            let pb = &self.particles[l.b + pid_start];
            self.links_js.push(LinkJS {
                ak: pa.k,
                bk: pb.k,
                p: (pa.p + pb.p) * 0.5,
            });
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

    pub fn link_js_size(&self) -> u32 {
        4 * 4
    }

    pub fn link_js_size_(&self) -> usize {
        4 * 4
    }

    pub fn links_js(&self) -> *const LinkJS {
        self.links_js.as_ptr()
    }

    pub fn links_count(&self) -> u32 {
        self.links.len() as u32
    }

    pub fn links_js_size(&self) -> u32 {
        (self.links_js.len() * self.link_js_size_()) as u32
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
            &mut self.links_js,
        );
        update_particles(
            self.diameter,
            &mut self.particles,
            &mut self.deltas,
            self.booster_acceleration,
        );
        for (sid, s) in self.ships_more.iter_mut().enumerate() {
            let pid0 = s.pids[0];
            let ship_control = &s.ship_control;
            let mut ship = &mut self.ships[sid];
            ship.pp = ship.p;
            ship.p = ship_position(&self.particles, s);
            // velocity
            let wa = wrap_around(ship.pp, ship.p);
            let speed = wa.d_sqrd.sqrt();
            if ship.on_target >= 1 && speed.abs() < self.max_speed_at_target {
                self.points += 1;
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

            match ship.anchor_pid {
                Some(x) => ship.target_pid = x,
                _ => {}
            };

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
            ship.orientation = ship_orientation(&self.particles, s);
            ship.vt = normalize_2(normalize_2(ship.td) + normalize_2(ship.v));
            ship.cross =
                normalize_2(normalize_2(ship.orientation) * 1.0 + normalize_2(ship.v) * 0.5);
            let orientation_angle = cross(normalize_2(ship.orientation), normalize_2(ship.td));
            let orientation_angle_2 = angle(normalize_2(ship.orientation), normalize_2(ship.td));
            let orientation_angle_corrected = cross(normalize_2(ship.cross), normalize_2(ship.td));
            let rotation_speed = cross(ship.orientation, previous_orientation);
            let rotation_speed_2 = angle(ship.orientation, previous_orientation);
            let velocity_vs_target_angle = cross(normalize_2(ship.v), normalize_2(ship.td));
            let mut action = "-";
            let forward_max_angle_better = radians(self.forward_max_angle / 2.0).sin();
            let slow_down_max_angle_better = radians(self.slow_down_max_angle / 2.0).sin();
            for pid in &s.pids {
                self.particles[*pid].a = 0;
            }

            match ship.anchor_pid {
                Some(anchor_pid) => {
                    let target_pid = 4;
                    let anchor = self.particles[anchor_pid].p;
                    let target = self.particles[target_pid].p;
                    let target_delta = wrap_around(ship.p, target);
                    let target_delta_old = wrap_around(ship.pp, target);
                    let target_to_anchor = wrap_around(anchor, target);
                    let target_to_anchor_distance = target_to_anchor.d_sqrd.sqrt();
                    let distance_to_target = target_delta.d_sqrd.sqrt();
                    let speed_toward_target = target_delta_old.d_sqrd.sqrt() - distance_to_target;
                    let target_direction = target_delta.d;
                    let anchor_delta = wrap_around(ship.p, anchor);
                    let distance_to_anchor = anchor_delta.d_sqrd.sqrt();
                    let anchor_delta_old = wrap_around(ship.pp, anchor);
                    let speed_toward_anchor = anchor_delta_old.d_sqrd.sqrt() - distance_to_anchor;
                    let anchor_direction = anchor_delta.d;
                    let orientation_angle =
                        cross(normalize_2(ship.orientation), normalize_2(target_direction));
                    let angle_aaa =
                        cross(normalize_2(anchor_direction), normalize_2(target_direction));
                    let action = if distance_to_target < target_to_anchor_distance
                        && orientation_angle.abs() < slow_down_max_angle_better
                        && speed_toward_target > -0.000_000_5
                    {
                        "slow down"
                    } else if orientation_angle > 0.0 && rotation_speed > -self.max_rotation_speed {
                        "turn left"
                    } else if orientation_angle < 0.0 && rotation_speed < self.max_rotation_speed {
                        "turn right"
                    } else if angle_aaa > 0.0
                        && distance_to_target < target_to_anchor_distance * 1.2
                        && speed_toward_anchor < 0.000_001
                    {
                        "translate right"
                    } else if angle_aaa < 0.0
                        && distance_to_target < target_to_anchor_distance * 1.2
                        && speed_toward_anchor < 0.000_001
                    {
                        "translate left"
                    } else {
                        "-"
                    };

                    // log(action);
                    // log(action);
                    // log(&format!("{}", angle_aaa));

                    match action {
                        "slow down" => {
                            for x in &ship_control.slow {
                                self.particles[pid0 + x].a = 1;
                            }
                        }
                        "turn left" | "turn left a" => {
                            for x in &ship_control.left {
                                self.particles[pid0 + x].a = 1;
                            }
                        }
                        "turn right" | "turn right a" => {
                            for x in &ship_control.right {
                                self.particles[pid0 + x].a = 1;
                            }
                        }
                        "forward" => {
                            for x in &ship_control.forward {
                                self.particles[pid0 + x].a = 1;
                            }
                        }
                        "translate left" => {
                            for x in &ship_control.translate_left {
                                self.particles[pid0 + x].a = 1;
                            }
                        }
                        "translate right" => {
                            for x in &ship_control.translate_right {
                                self.particles[pid0 + x].a = 1;
                            }
                        }
                        _ => {}
                    }
                }
                _ => {
                    if orientation_angle_corrected.abs() < slow_down_max_angle_better
                        && speed_toward_target
                            > (self.max_speed_at_target * 0.75)
                                .max(distance_to_target * self.slow_down_max_speed_to_target_ratio)
                    {
                        action = "slow down";
                        for x in &ship_control.slow {
                            self.particles[pid0 + x].a = 1;
                        }
                    } else if orientation_angle_corrected > 0.0
                        && rotation_speed > -self.max_rotation_speed
                    {
                        action = "turn left";
                        for x in &ship_control.left {
                            self.particles[pid0 + x].a = 1;
                        }
                    } else if orientation_angle_corrected < 0.0
                        && rotation_speed < self.max_rotation_speed
                    {
                        action = "turn right";
                        for x in &ship_control.right {
                            self.particles[pid0 + x].a = 1;
                        }
                    } else if speed < self.forward_max_speed
                        && orientation_angle_corrected.abs() < forward_max_angle_better
                    {
                        action = "forward";
                        for x in &ship_control.forward {
                            self.particles[pid0 + x].a = 1;
                        }
                    }
                }
            };

            if sid == usize::MAX {
                log(&format!(
                    "forward_max_angle_better: {}\ndistance_to_target: {}\norientation_angle: {}\norientation_angle_2: {}\norientation_angle_corrected: {}\nrotation_speed: {}\nrotation_speed_2: {}\nspeed: {}\nvelocity_vs_target_angle: {}\nspeed_toward_target: {}\naction:{}",
                    forward_max_angle_better,
                    distance_to_target,
                    orientation_angle,
                    orientation_angle_2,
                    orientation_angle_corrected,
                    rotation_speed,
                    rotation_speed_2,
                    speed,
                    velocity_vs_target_angle, speed_toward_target, action
                ));
            }
        }
        self.step += 1;
    }
}
