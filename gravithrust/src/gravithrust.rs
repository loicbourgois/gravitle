use crate::blueprint::load_raw_blueprint;
use crate::blueprint::Blueprint;
use crate::blueprint::RawBlueprint;
use crate::gravithrust_tick::add_particle;
use crate::gravithrust_tick::add_particle_2;
use crate::gravithrust_tick::add_particles;
use crate::gravithrust_tick::compute_collision_responses;
use crate::gravithrust_tick::compute_link_responses;
use crate::gravithrust_tick::update_particles;
use crate::grid::Grid;
use crate::job::Action;
use crate::job::Condition;
use crate::job::Job;
// use crate::job::Task;
use crate::kind::kindstr_to_kind;
use crate::kind::Kind;
use crate::link::Link;
use crate::link::LinkJS;
#[allow(unused_imports)]
use crate::log;
use crate::math::angle;
use crate::math::cross;
use crate::math::normalize_2;
use crate::math::radians;
use crate::math::wrap_around;
use crate::math::Vector;
use crate::math::WrapAroundResponse;
use crate::particle::Particle;
use crate::particle::ParticleInternal;
use crate::particle::Particles;
use crate::ship::ship_orientation;
use crate::ship::ship_position;
use crate::ship::Ship;
use crate::ship::ShipControl;
use crate::ship::ShipMore;
use rand::Rng;
use wasm_bindgen::prelude::wasm_bindgen;
#[wasm_bindgen]
pub struct Gravithrust {
    particles: Vec<Particle>,
    ships: Vec<Ship>,
    links: Vec<Link>,
    links_js: Vec<LinkJS>,
    particles_internal: Vec<ParticleInternal>,
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
        Gravithrust {
            particles: vec![],
            links: vec![],
            particles_internal: vec![],
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
        }
    }

    pub fn add_ship(&mut self, yml_blueprint: &str, x: f32, y: f32) -> usize {
        let raw_blueprint: RawBlueprint = serde_yaml::from_str(yml_blueprint).unwrap();
        let blueprint = load_raw_blueprint(&raw_blueprint, self.diameter);
        self.add_ship_internal(
            &blueprint,
            Vector {
                x,
                y,
            },
        )
    }

    pub fn add_structure(&mut self, yml_blueprint: &str, x: f32, y: f32) -> usize {
        let raw_blueprint: RawBlueprint = serde_yaml::from_str(yml_blueprint).unwrap();
        let blueprint = load_raw_blueprint(&raw_blueprint, self.diameter);
        self.add_structure_internal(
            &blueprint,
            Vector {
                x,
                y,
            },
            None,
        )[0]
    }

    pub fn add_particle(&mut self, x: f32, y: f32, kind: &str) -> usize {
        self.add_particle_inner(
            Vector {
                x,
                y,
            },
            kindstr_to_kind(kind),
            None,
        )
    }

    pub fn add_particle_inner(&mut self, p: Vector, k: Kind, sid: Option<usize>) -> usize {
        add_particle(&mut self.particles, &mut self.particles_internal, p, k, sid)
    }

    pub fn set_anchor(&mut self, sid: usize, pid: usize) {
        self.ships_more[sid].anchor_pid = Some(pid);
    }

    pub fn set_target(&mut self, sid: usize, pid: usize) {
        self.ships_more[sid].target_pid = Some(pid);
    }

    pub fn add_structure_internal(
        &mut self,
        blueprint: &Blueprint,
        position: Vector,
        sid: Option<usize>,
    ) -> Vec<usize> {
        let pid_start = self.particles.len();
        let mut pids = vec![];
        for p in &blueprint.particles {
            pids.push(self.particles.len());
            self.add_particle_inner(p.p + position, p.k, sid);
        }
        for l in &blueprint.links {
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
        pids
    }

    pub fn add_ship_internal(&mut self, blueprint: &Blueprint, position: Vector) -> usize {
        let sid = Some(self.ships.len());
        let ship = Ship {
            target: Vector {
                x: rand::thread_rng().gen::<f32>(),
                y: rand::thread_rng().gen::<f32>(),
            },
            p: Vector::default(),
            pp: Vector::default(),
            v: Vector::default(),
            td: Vector::default(),
            orientation: Vector::default(),
            vt: Vector::default(),
            cross: Vector::default(),
            on_target: 0,
        };
        let pids = self.add_structure_internal(blueprint, position, sid);
        let ship_more = ShipMore {
            pids,
            ship_control: ShipControl {
                left: blueprint.left.clone(),
                right: blueprint.right.clone(),
                slow: blueprint.slow.clone(),
                forward: blueprint.forward.clone(),
                translate_left: blueprint.translate_left.clone(),
                translate_right: blueprint.translate_right.clone(),
            },
            anchor_pid: None,
            target_pid: None,
            job: None,
        };
        self.ships.push(ship);
        self.ships[sid.unwrap()].p = ship_position(&self.particles, &ship_more);
        self.ships_more.push(ship_more);
        self.ships[sid.unwrap()].pp = self.ships[sid.unwrap()].p;
        sid.unwrap()
    }

    pub fn particles_size(&self) -> u32 {
        (self.particles.len() * Gravithrust::particle_size_internal()) as u32
    }

    pub fn particle_size(&self) -> u32 {
        Gravithrust::particle_size_internal() as u32
    }

    pub fn particle_size_internal() -> usize {
        15 * 4
    }

    pub fn particles_count(&self) -> u32 {
        self.particles.len() as u32
    }

    pub fn particles(&self) -> *const Particle {
        self.particles.as_ptr()
    }

    pub fn ships_size(&self) -> u32 {
        (self.ships.len() * Gravithrust::ship_size_internal()) as u32
    }

    pub fn ship_size(&self) -> u32 {
        Gravithrust::ship_size_internal() as u32
    }

    pub fn ship_size_internal() -> usize {
        17 * 4
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

    pub fn ticks(&mut self) {
        for _ in 0..self.sub_steps {
            self.tick();
        }
    }

    pub fn set_job(&mut self, sid: usize, job_json: &str) {
        self.set_job_internal(sid, serde_json::from_str(job_json).unwrap());
    }
}
impl Gravithrust {
    pub fn set_job_internal(&mut self, sid: usize, job: Job) {
        self.ships_more[sid].job = Some(job);
    }

    fn tick(&mut self) {
        add_particles(
            self.diameter,
            &mut self.particles,
            &mut self.particles_internal,
        );
        self.grid.update_01();
        self.grid.update_02(&mut self.particles);
        compute_collision_responses(
            self.diameter,
            &mut self.particles,
            &mut self.particles_internal,
            &self.grid,
        );
        compute_link_responses(
            self.diameter,
            &mut self.particles,
            &mut self.particles_internal,
            &mut self.links,
            &mut self.links_js,
        );
        update_particles(
            self.diameter,
            &mut self.particles,
            &mut self.particles_internal,
            self.booster_acceleration,
        );
        self.update_ships();
        self.step += 1;
    }

    fn check_job(&mut self, sid: usize) {
        let mut ship_more = &mut self.ships_more[sid];
        let ship = &self.ships[sid];
        match &ship_more.job {
            Some(job) => {
                for task in &job.tasks {
                    let mut ok = true;
                    for condition in &task.conditions {
                        match condition {
                            Condition::StorageNotFull => {
                                let mut capacity = 0;
                                let mut volume = 0;
                                for pid in &ship_more.pids {
                                    let p = &self.particles[*pid];
                                    volume += p.volume;
                                    capacity += p.k.soft_capacity();
                                }
                                if volume >= capacity {
                                    ok = false;
                                }
                            }
                            Condition::StorageFull => {
                                let mut capacity = 0;
                                let mut volume = 0;
                                for pid in &ship_more.pids {
                                    let p = &self.particles[*pid];
                                    volume += p.volume;
                                    capacity += p.k.soft_capacity();
                                }
                                if volume < capacity {
                                    ok = false;
                                }
                            }
                        }
                        if !ok {
                            continue;
                        }
                    }
                    if ok {
                        match task.action {
                            Action::CollectElectroFieldPlasma => match ship_more.target_pid {
                                None => {
                                    let mut dmin = 100.0;
                                    let mut target_pid = None;
                                    for p in &self.particles {
                                        if p.k == Kind::ElectroFieldPlasma {
                                            let wa = wrap_around(p.p, ship.p);
                                            if wa.d_sqrd < dmin {
                                                dmin = wa.d_sqrd;
                                                target_pid = Some(p.idx);
                                            }
                                        }
                                    }
                                    ship_more.target_pid = target_pid;
                                    match target_pid {
                                        Some(pid) => {
                                            let p = &self.particles[pid];
                                            log(&format!("s#{} -> p#{}:{:?}", sid, p.idx, p.k));
                                        }
                                        None => {}
                                    }
                                }
                                Some(target_pid) => {
                                    if self.particles[target_pid].k != Kind::ElectroFieldPlasma {
                                        ship_more.target_pid = None;
                                    }
                                }
                            },
                            Action::DeliverPlasma => match ship_more.target_pid {
                                None => {
                                    for p in &self.particles {
                                        if p.k == Kind::PlasmaDepot {
                                            ship_more.target_pid = Some(p.idx);
                                            log(&format!("s#{} -> p#{}:{:?}", sid, p.idx, p.k));
                                            break;
                                        }
                                    }
                                }
                                Some(target_pid) => {
                                    if self.particles[target_pid].k != Kind::PlasmaDepot {
                                        ship_more.target_pid = None;
                                    }
                                }
                            },
                            Action::ResetTarget => {
                                ship_more.target_pid = None;
                            }
                        }
                        break;
                    }
                }
            }
            None => {}
        }
    }

    fn update_ships(&mut self) {
        let l = self.ships_more.len();
        for sid in 0..l {
            self.check_job(sid);
            let s = &mut self.ships_more[sid];
            let mut ship = &mut self.ships[sid];
            match s.target_pid {
                Some(pid) => {
                    ship.target = self.particles[pid].p;
                }
                None => {}
            };
            ship.pp = ship.p;
            ship.p = ship_position(&self.particles, s);
            let position_wa = wrap_around(ship.pp, ship.p);
            let speed = position_wa.d_sqrd.sqrt();
            ship.v = position_wa.d;
            let target_delta_wa = wrap_around(ship.p, ship.target);
            ship.td = target_delta_wa.d;
            let previous_orientation = ship.orientation;
            ship.orientation = ship_orientation(&self.particles, s);
            ship.vt = normalize_2(normalize_2(ship.td) + normalize_2(ship.v));
            ship.cross =
                normalize_2(normalize_2(ship.orientation) * 1.0 + normalize_2(ship.v) * 0.5);
            let slow_down_max_angle_better = radians(self.slow_down_max_angle / 2.0).sin();
            let rotation_speed = cross(ship.orientation, previous_orientation);
            let movement_action = match (s.anchor_pid, s.target_pid) {
                (Some(anchor_pid), Some(target_pid)) => anchor_target_movement_action(
                    rotation_speed,
                    self.max_rotation_speed,
                    slow_down_max_angle_better,
                    ship,
                    &self.particles,
                    anchor_pid,
                    target_pid,
                ),
                (None, Some(target_pid)) => target_only_movement_action(
                    self.forward_max_speed,
                    self.max_speed_at_target,
                    self.slow_down_max_speed_to_target_ratio,
                    self.forward_max_angle,
                    self.max_rotation_speed,
                    speed,
                    rotation_speed,
                    self.slow_down_max_angle,
                    &self.particles,
                    ship,
                    &target_delta_wa,
                    target_pid,
                ),
                _ => "-",
            };
            for pid in &s.pids {
                self.particles[*pid].a = 0;
            }
            apply_movement_action(&mut self.particles, movement_action, s);
            match (s.anchor_pid, s.target_pid) {
                (Some(anchor_pid), Some(target_pid)) => {
                    let anchor = self.particles[anchor_pid].p;
                    let target = self.particles[target_pid].p;
                    let anchor_delta = wrap_around(ship.p, anchor);
                    let ray_particle = &mut self.particles[s.pids[0]];
                    let uu = normalize_2(wrap_around(ship.p, target).d);
                    if anchor_delta.d_sqrd < 0.001
                        && ray_particle.e >= 2500
                        && angle(uu, normalize_2(ray_particle.direction)).abs() < 0.01
                    {
                        ray_particle.e = 0;
                        let aa = ray_particle.p + ray_particle.direction * self.diameter * 1.75;
                        add_particle_2(
                            &mut self.particles,
                            &mut self.particles_internal,
                            aa,
                            uu * self.diameter * 0.01,
                            Kind::ElectroField,
                            None,
                        );
                    }
                }
                _ => {}
            }
        }
    }
}
fn apply_movement_action(particles: &mut Particles, movement_action: &str, ship_more: &ShipMore) {
    let pid0 = ship_more.pids[0];
    match movement_action {
        "slow down" => {
            for x in &ship_more.ship_control.slow {
                particles[pid0 + x].a = 1;
            }
        }
        "turn left" => {
            for x in &ship_more.ship_control.left {
                particles[pid0 + x].a = 1;
            }
        }
        "turn right" => {
            for x in &ship_more.ship_control.right {
                particles[pid0 + x].a = 1;
            }
        }
        "forward" => {
            for x in &ship_more.ship_control.forward {
                particles[pid0 + x].a = 1;
            }
        }
        "translate left" => {
            for x in &ship_more.ship_control.translate_left {
                particles[pid0 + x].a = 1;
            }
        }
        "translate right" => {
            for x in &ship_more.ship_control.translate_right {
                particles[pid0 + x].a = 1;
            }
        }
        _ => {}
    };
}
fn anchor_target_movement_action<'a>(
    rotation_speed: f32,
    max_rotation_speed: f32,
    slow_down_max_angle_better: f32,
    ship: &Ship,
    particles: &Particles,
    anchor_pid: usize,
    target_pid: usize,
) -> &'a str {
    let anchor = particles[anchor_pid].p;
    let target = particles[target_pid].p;
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
    let orientation_angle = cross(normalize_2(ship.orientation), normalize_2(target_direction));
    let angle_anchor_to_target =
        cross(normalize_2(anchor_direction), normalize_2(target_direction));
    if distance_to_target < target_to_anchor_distance
        && orientation_angle.abs() < slow_down_max_angle_better
        && speed_toward_target > -0.000_000_5
    {
        "slow down"
    } else if orientation_angle > 0.0 && rotation_speed > -max_rotation_speed {
        "turn left"
    } else if orientation_angle < 0.0 && rotation_speed < max_rotation_speed {
        "turn right"
    } else if angle_anchor_to_target > 0.0
        && distance_to_target < target_to_anchor_distance * 1.2
        && speed_toward_anchor < 0.000_001
    {
        "translate right"
    } else if angle_anchor_to_target < 0.0
        && distance_to_target < target_to_anchor_distance * 1.2
        && speed_toward_anchor < 0.000_001
    {
        "translate left"
    } else {
        "-"
    }
}
fn target_only_movement_action<'a>(
    forward_max_speed: f32,
    max_speed_to_target: f32,
    slow_down_max_speed_to_target_ratio: f32,
    forward_max_angle: f32,
    max_rotation_speed: f32,
    speed: f32,
    rotation_speed: f32,
    slow_down_max_angle: f32,
    particles: &Particles,
    ship: &mut Ship,
    target_delta_wa: &WrapAroundResponse,
    target_id: usize,
) -> &'a str {
    let pt = &particles[target_id];
    let wa1 = wrap_around(ship.pp, pt.pp);
    let wa2 = wrap_around(ship.p, pt.p);
    let target_vs_ship_delta_v = wa1.d_sqrd.sqrt() - wa2.d_sqrd.sqrt();
    let distance_to_target = target_delta_wa.d_sqrd.sqrt();
    let orientation_angle_corrected_2 = angle(normalize_2(ship.cross), normalize_2(ship.td));
    let orientation_angle_corrected = cross(normalize_2(ship.cross), normalize_2(ship.td));
    if orientation_angle_corrected_2.abs() < slow_down_max_angle / 2.0
        && target_vs_ship_delta_v
            > (max_speed_to_target * 0.75)
                .max(distance_to_target * slow_down_max_speed_to_target_ratio)
    {
        "slow down"
    } else if orientation_angle_corrected > 0.0 && rotation_speed > -max_rotation_speed {
        "turn left"
    } else if orientation_angle_corrected < 0.0 && rotation_speed < max_rotation_speed {
        "turn right"
    } else if speed < forward_max_speed
        && orientation_angle_corrected_2.abs() < forward_max_angle / 2.0
    {
        "forward"
    } else {
        "-"
    }
}
