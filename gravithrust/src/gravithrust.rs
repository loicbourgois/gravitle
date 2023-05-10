use crate::alchemy_generated::alchemy_transform;
use crate::blueprint::load_raw_blueprint;
use crate::blueprint::Blueprint;
use crate::blueprint::RawBlueprint;
use crate::elapsed_secs_f32;
use crate::gravithrust_tick::compute_collision_responses;
use crate::gravithrust_tick::compute_link_responses;
use crate::grid::Grid;
use crate::job::Job;
use crate::kind::kindstr_to_kind;
use crate::kind::Kind;
use crate::link::Link;
use crate::link::LinkJS;
use crate::log;
use crate::math::cross;
use crate::math::normalize_2;
use crate::math::radians;
use crate::math::wrap_around;
use crate::math::Vector;
use crate::movement::apply_movement_action;
use crate::now;
use crate::particle::Particle;
use crate::particle::ParticleInternal;
use crate::particle::Quantities;
use crate::particle::QuantityKind;
use crate::ship::ship_orientation;
use crate::ship::ship_position;
use crate::ship::Ship;
use crate::ship::ShipControl;
use crate::ship::ShipMore;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use wasm_bindgen::prelude::wasm_bindgen;
#[derive(Debug, Serialize, Deserialize)]
pub struct GravithrustState {
    pub capacity: HashMap<Kind, HashMap<QuantityKind, u32>>,
    pub quantity: HashMap<Kind, HashMap<QuantityKind, u32>>,
    pub count: HashMap<Kind, u32>,
}
#[wasm_bindgen]
pub struct Gravithrust {
    #[wasm_bindgen(skip)]
    pub live_particles: HashSet<usize>,
    #[wasm_bindgen(skip)]
    pub dead_particles: HashSet<usize>,
    #[wasm_bindgen(skip)]
    pub particles: Vec<Particle>,
    #[wasm_bindgen(skip)]
    pub ships: Vec<Ship>,
    #[wasm_bindgen(skip)]
    pub links: Vec<Link>,
    #[wasm_bindgen(skip)]
    pub links_js: Vec<LinkJS>,
    #[wasm_bindgen(skip)]
    pub particles_internal: Vec<ParticleInternal>,
    #[wasm_bindgen(skip)]
    pub ships_more: Vec<ShipMore>,
    pub diameter: f32,
    #[wasm_bindgen(skip)]
    pub grid: Grid,
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
    #[wasm_bindgen(skip)]
    pub state: GravithrustState,
    #[wasm_bindgen(skip)]
    pub durations: VecDeque<Duration>,
    pub avg_duration: Duration,
    #[wasm_bindgen(skip)]
    pub avg_durations: VecDeque<Duration>,
    #[wasm_bindgen(skip)]
    pub avg_durations_vec: Vec<Duration>,
}
#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Duration {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: f32,
    pub f: f32,
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
        log("new Gravithrust");
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
            state: GravithrustState {
                capacity: HashMap::new(),
                count: HashMap::new(),
                quantity: HashMap::new(),
            },
            live_particles: HashSet::new(),
            dead_particles: HashSet::new(),
            durations: VecDeque::new(),
            avg_duration: Duration {
                a: 0.0,
                b: 0.0,
                c: 0.0,
                d: 0.0,
                e: 0.0,
                f: 0.0,
            },
            avg_durations: VecDeque::new(),
            avg_durations_vec: Vec::new(),
        }
    }

    pub fn print_particle(&self, pid: usize) {
        log(&format!("{:?}", self.particles[pid]));
    }

    pub fn get_particle_kind(&self, pid: usize) -> u32 {
        self.particles[pid].k as u32
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
        self.add_particle_internal(
            Vector {
                x,
                y,
            },
            kindstr_to_kind(kind),
            None,
        )
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
            self.add_particle_internal(p.p + position, p.k, sid);
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
                x: 0.5,
                y: 0.5,
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
            sid: sid.unwrap(),
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
        20 * 4
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

    pub fn average_durations_pointer(&self) -> *const Duration {
        self.avg_durations_vec.as_ptr()
    }

    pub fn average_durations_size_unit(&self) -> u32 {
        6 * 4
    }

    pub fn average_durations_size_full(&self) -> u32 {
        self.average_durations_size_unit() * self.avg_durations_vec.len() as u32
    }

    pub fn average_durations_count(&self) -> u32 {
        self.avg_durations_vec.len() as u32
    }

    pub fn ticks(&mut self) -> String {
        for _ in 0..self.sub_steps {
            self.tick();
        }
        self.update_state();
        while self.durations.len() > 10000 {
            self.durations.pop_front();
        }
        self.avg_duration = Duration {
            a: 0.0,
            b: 0.0,
            c: 0.0,
            d: 0.0,
            e: 0.0,
            f: 0.0,
        };
        for duration in &self.durations {
            self.avg_duration.a += duration.a;
            self.avg_duration.b += duration.b;
            self.avg_duration.c += duration.c;
            self.avg_duration.d += duration.d;
            self.avg_duration.e += duration.e;
            self.avg_duration.f += duration.f;
        }
        self.avg_duration.a /= self.durations.len() as f32;
        self.avg_duration.b /= self.durations.len() as f32;
        self.avg_duration.c /= self.durations.len() as f32;
        self.avg_duration.d /= self.durations.len() as f32;
        self.avg_duration.e /= self.durations.len() as f32;
        self.avg_duration.f /= self.durations.len() as f32;
        self.avg_durations.push_back(self.avg_duration);
        while self.avg_durations.len() > 1000 {
            self.avg_durations.pop_front();
        }
        self.avg_durations_vec = self.avg_durations.clone().into();
        self.get_state()
    }

    pub fn set_job(&mut self, sid: usize, job_json: &str) {
        self.set_job_internal(sid, serde_json::from_str(job_json).unwrap());
    }
}
impl Gravithrust {
    pub fn add_particle_internal_2(
        &mut self,
        position: Vector,
        velocity: Vector,
        k: Kind,
        sid: Option<usize>,
    ) -> usize {
        let pid = if self.dead_particles.is_empty() {
            let pid = self.particles.len();
            self.particles.push(Particle::default());
            self.particles_internal.push(ParticleInternal::default());
            pid
        } else {
            let pid: usize = *self.dead_particles.iter().next().unwrap();
            assert!(self.dead_particles.remove(&pid));
            pid
        };
        self.particles[pid] = Particle {
            p: position,
            pp: position - velocity,
            v: velocity,
            m: 1.0,
            k,
            direction: Vector::default(),
            a: 0,
            idx: pid,
            grid_id: 0,
            qs: Quantities::default(),
            live: 1,
        };
        self.particles_internal[pid] = ParticleInternal {
            dp: Vector::default(),
            dv: Vector::default(),
            direction: Vector::default(),
            sid,
            new_state: None,
        };
        self.live_particles.insert(pid);
        pid
    }

    pub fn update_state(&mut self) {
        self.state.count.clear();
        self.state.capacity.clear();
        self.state.quantity.clear();
        for p in &self.particles {
            *self.state.count.entry(p.k).or_insert(0) += 1;
            let capacities = self.state.capacity.entry(p.k).or_insert(HashMap::new());
            let quantities = self.state.quantity.entry(p.k).or_insert(HashMap::new());
            for qk in p.qks() {
                *capacities.entry(*qk).or_insert(0) += p.capacity(*qk);
                *quantities.entry(*qk).or_insert(0) += p.quantity(*qk);
            }
        }
    }

    pub fn get_state(&mut self) -> String {
        serde_json::to_string(&self.state).unwrap()
    }

    pub fn set_job_internal(&mut self, sid: usize, job: Job) {
        self.ships_more[sid].job = Some(job);
    }

    pub fn alchemy_transform(&mut self) {
        for (pid, p1) in self.particles.iter_mut().enumerate() {
            let pi1 = &mut self.particles_internal[pid];
            alchemy_transform(p1, pi1);
        }
    }

    pub fn tick(&mut self) {
        let ia = now();
        self.grid.update_01();
        self.grid.update_02(&mut self.particles);
        let a = elapsed_secs_f32(ia);
        let ib = now();
        self.alchemy_transform();
        let b = elapsed_secs_f32(ib);
        let ic = now();
        compute_collision_responses(
            self.diameter,
            &mut self.particles,
            &mut self.particles_internal,
            &self.grid,
        );
        let c = elapsed_secs_f32(ic);
        let id = now();
        compute_link_responses(
            self.diameter,
            &mut self.particles,
            &mut self.particles_internal,
            &mut self.links,
            &mut self.links_js,
        );
        let d = elapsed_secs_f32(id);
        let ie = now();
        self.update_particles();
        let e = elapsed_secs_f32(ie);
        let i_f = now();
        self.update_ships();
        let f = elapsed_secs_f32(i_f);
        self.durations.push_back(Duration {
            a,
            b,
            c,
            d,
            e,
            f,
        });
        self.step += 1;
    }

    pub fn update_ships(&mut self) {
        let slow_down_max_angle_better = radians(self.slow_down_max_angle / 2.0).sin();
        let l = self.ships_more.len();
        for sid in 0..l {
            let ship_more = &self.ships_more[sid];
            let previous_orientation = self.ships[sid].orientation;
            {
                let mut ship = &mut self.ships[sid];
                match ship_more.target_pid {
                    Some(pid) => {
                        ship.target = self.particles[pid].p;
                    }
                    None => {}
                };
                ship.pp = ship.p;
                ship.p = ship_position(&self.particles, ship_more);
                ship.v = wrap_around(ship.pp, ship.p).d;
                ship.td = wrap_around(ship.p, ship.target).d;
                ship.orientation = ship_orientation(&self.particles, ship_more);
                ship.vt = normalize_2(normalize_2(ship.td) + normalize_2(ship.v));
                ship.cross =
                    normalize_2(normalize_2(ship.orientation) * 1.0 + normalize_2(ship.v) * 0.5);
            }
            let ship = &self.ships[sid];
            let speed = wrap_around(ship.pp, ship.p).d_sqrd.sqrt();
            let rotation_speed = cross(ship.orientation, previous_orientation);
            let target_delta_wa = wrap_around(ship.p, ship.target);
            let movement_action = self.get_movement_action(
                &target_delta_wa,
                ship,
                rotation_speed,
                slow_down_max_angle_better,
                ship_more,
                speed,
            );
            apply_movement_action(&mut self.particles, movement_action, ship_more);
            self.check_job(sid);
        }
    }
}
