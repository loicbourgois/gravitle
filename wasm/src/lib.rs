mod collision;
mod maths;
mod plan;
mod point;
mod utils;
use crate::{
    collision::collision,
    maths::{delta_position_wrap_around, distance_squared_wrap_around, dot, normalize, p_coords},
    plan::Plan,
    point::Point,
    utils::set_panic_hook,
};
use rand;
use std::collections::HashMap;
use std::sync::Mutex;
use utils::log;
use uuid::Uuid;
use wasm_bindgen::prelude::wasm_bindgen;
const BASE_CAPACITY: usize = 2;
const DIAMETER: Float = 0.01;
const MASS: Float = 1.0;
const LINK_STRENGTH: Float = 1000.0;
type Float = f32;
#[wasm_bindgen]
#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum Kind {
    Firefly = 1,
    Metal = 2,
    Turbo = 3,
    Diatom = 4,
    Neuron = 5,
    Mouth = 6,
}
#[wasm_bindgen]
#[repr(u32)]
#[derive(Clone, Copy, Debug, Hash, std::cmp::Eq, std::cmp::PartialEq)]
pub enum DataKind {
    Activity = 1,
    Water = 2,
}
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Part {
    p: Point,  // position
    pp: Point, // previous position
    d: Float,  // diameter
    m: Float,  // mass
    kind: Kind,
    trace_a: Float,
    trace_b: Float,
    trace_c: Float,
    trace_d: Float,
    // activity: Float,
}
#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Blocks {
    w: usize,
    h: usize,
}
#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Block {
    x: usize,
    y: usize,
}
#[wasm_bindgen]
pub struct Server {
    blocks: Blocks,
    parts: HashMap<u128, Part>,
    datas: HashMap<u128, HashMap<DataKind, Float>>,
    // parts_data: HashMap<u128, PartData>,
    parts_vec: Vec<Part>,
    pids: Vec<Vec<Vec<u128>>>,
    step: usize,
    links: HashMap<u128, HashMap<u128, Float>>,
}

// enum PartData {
//     Firefly,
//     Metal ,
//     Turbo {
//         activity: Float,
//     },
//     Diatom ,
//     Neuron {
//         activity: Float,
//     },
//     Mouth,
// }

#[wasm_bindgen]
impl Block {
    pub fn new(p: &Point, b: &Blocks) -> Block {
        Block {
            x: ((p.x * b.w as Float) % b.w as Float).floor() as usize,
            y: ((p.y * b.h as Float) % b.h as Float).floor() as usize,
        }
    }
}

#[wasm_bindgen]
impl Server {
    pub fn new(width: usize, height: usize) -> Server {
        set_panic_hook();
        Server {
            blocks: Blocks {
                w: width,
                h: height,
            },
            parts: HashMap::new(),
            parts_vec: Vec::new(),
            pids: vec![vec![Vec::with_capacity(BASE_CAPACITY); height]; width],
            step: 0,
            links: HashMap::new(),
            datas: HashMap::new(),
            //mutex: Mutex::new(false)
        }
    }

    pub fn add_part(&mut self, kind: Kind, p: &Point, v: &Point) {
        self.add_part_(kind, p, v);
    }

    fn add_part_(&mut self, kind: Kind, p: &Point, v: &Point) -> u128 {
        let block = Block::new(&p, &self.blocks);
        let pid: u128 = Uuid::new_v4().as_u128();
        self.parts.insert(
            pid,
            Part {
                p: *p,
                pp: p - v,
                d: DIAMETER,
                m: MASS,
                kind: kind,
                trace_a: 0.0,
                trace_b: 0.0,
                trace_c: 0.0,
                trace_d: 0.0,
            },
        );
        let mut data_map = HashMap::new();

        match kind {
            Kind::Turbo => {
                data_map.insert(DataKind::Activity, 0.0);
            }
            Kind::Neuron => {
                data_map.insert(DataKind::Activity, 0.0);
            }
            _ => {}
        }
        self.datas.insert(pid, data_map);
        self.pids[block.x][block.y].push(pid);
        pid
    }

    fn add_link(&mut self, pid1: &u128, pid2: &u128, strengh: Float) {
        let (pid_min, pid_max) = if pid1 < pid2 {
            (pid1, pid2)
        } else {
            (pid2, pid1)
        };
        match self.links.get_mut(pid_min) {
            Some(a) => match a.get_mut(pid_max) {
                Some(b) => {
                    log("should not happen");
                }
                None => {
                    a.insert(*pid_max, strengh);
                }
            },
            None => {
                let mut hashmap = HashMap::new();
                hashmap.insert(*pid_max, strengh);
                self.links.insert(*pid_min, hashmap);
            }
        }
    }

    pub fn add_entity(&mut self, p: &Plan, c: &Point) {
        let v = Point { x: 0.0, y: 0.0 };
        let p1 = c - &Point {
            x: DIAMETER * 0.5,
            y: 0.0,
        };
        let p2 = c + &Point {
            x: DIAMETER * 0.5,
            y: 0.0,
        };
        let mut coords = Vec::new();
        let mut pids = Vec::new();
        coords.push(p1);
        coords.push(p2);
        pids.push(self.add_part_(p.k1, &p1, &v));
        pids.push(self.add_part_(p.k2, &p2, &v));
        self.add_link(&pids[0], &pids[1], LINK_STRENGTH);
        for part in p.part_plans_().iter() {
            let pos = p_coords(&coords[part.a], &coords[part.b]);
            let pid1 = self.add_part_(part.k, &pos, &v);
            let p1 = self.parts[&pid1];
            for pid2 in pids.iter() {
                let p2 = self.parts[&pid2];
                let d_sqrd = distance_squared_wrap_around(&p1.p, &p2.p);
                let diams = (p1.d + p2.d) * 0.5;
                if d_sqrd < diams * diams * 1.1 {
                    self.add_link(&pid1, &pid2, LINK_STRENGTH);
                }
            }
            pids.push(pid1);
            coords.push(pos);
        }
    }

    pub fn tick(&mut self) {
        let mut new_parts: HashMap<u128, Part> = HashMap::new();
        let mut new_datas: HashMap<u128, HashMap<DataKind, Float>> = HashMap::new();
        let mut new_pids: Vec<Vec<Vec<u128>>> =
            vec![vec![Vec::with_capacity(BASE_CAPACITY); self.blocks.h]; self.blocks.w];
        for i in 0..self.blocks.w {
            for j in 0..self.blocks.h {
                for pid1 in self.pids[i][j].iter() {
                    let p1 = self.parts.get(pid1).unwrap();
                    let d1 = self.datas.get(pid1).unwrap();
                    let mut dpc = Point { x: 0.0, y: 0.0 };
                    let mut force = Point { x: 0.0, y: 0.0 };
                    let mut direction = Point { x: 0.0, y: 0.0 };
                    let mut activity_sum = 0.0;
                    let mut activity_count = 0.0;
                    let i2min = (i + self.blocks.w - 1) % self.blocks.w;
                    let i2max = (i + self.blocks.w + 1) % self.blocks.w;
                    for i2 in [i2min, i, i2max] {
                        let j2min = (j + self.blocks.h - 1) % self.blocks.h;
                        let j2max = (j + self.blocks.h + 1) % self.blocks.h;
                        for j2 in [j2min, j, j2max] {
                            for pid2 in self.pids[i2][j2].iter() {
                                if pid1 != pid2 {
                                    let p2 = self.parts.get(pid2).unwrap();
                                    let d2 = self.datas.get(pid2).unwrap();
                                    let collision_response = collision(p1, p2, self, pid1, pid2);
                                    force += &collision_response.force;
                                    dpc += &collision_response.delta_position;
                                    if collision_response.linked {
                                        direction += &delta_position_wrap_around(&p1.p, &p2.p);
                                        match (
                                            d1.get(&DataKind::Activity),
                                            d2.get(&DataKind::Activity),
                                        ) {
                                            (Some(_activity1), Some(activity2)) => {
                                                activity_count += 1.0;
                                                activity_sum += activity2;
                                            }
                                            _ => {}
                                        }
                                        //
                                    }
                                }
                            }
                        }
                    }

                    let mut new_data = HashMap::new();

                    match p1.kind {
                        Kind::Neuron => {
                            let delta_activity = 0.1;
                            let old_activity = d1.get(&DataKind::Activity).unwrap();
                            let new_activity = (old_activity - delta_activity * 0.5
                                + rand::random::<Float>() * delta_activity)
                                .max(-1.0)
                                .min(1.0);
                            new_data.insert(DataKind::Activity, new_activity);
                        }
                        Kind::Turbo => {
                            new_data.insert(DataKind::Activity, activity_sum / activity_count);
                        }
                        _ => {}
                    };
                    let trace_a: Float;
                    let trace_b: Float;
                    let trace_c: Float;
                    let trace_d: Float;

                    match p1.kind {
                        Kind::Turbo => {
                            trace_a = *d1.get(&DataKind::Activity).unwrap();
                            trace_b = 0.0;
                            trace_c = 0.0;
                            trace_d = 0.0;
                        }
                        Kind::Neuron => {
                            trace_a = *d1.get(&DataKind::Activity).unwrap();
                            trace_b = 0.0;
                            trace_c = 0.0;
                            trace_d = 0.0;
                        }
                        _ => {
                            trace_a = 0.0;
                            trace_b = 0.0;
                            trace_c = 0.0;
                            trace_d = 0.0;
                        }
                    };

                    match p1.kind {
                        Kind::Turbo => {
                            let n = normalize(&direction);
                            let strength =
                                0.001 * d1.get(&DataKind::Activity).unwrap().max(0.0).min(1.0);
                            if n.x.is_finite() && n.y.is_finite() {
                                force.x -= n.x * strength;
                                force.y -= n.y * strength;
                            }
                        }
                        _ => {}
                    };
                    let acc = Point {
                        x: force.x / p1.m,
                        y: force.y / p1.m,
                    };
                    let delta_time = 1.0 / 60.0;
                    let v = Point {
                        x: p1.p.x - p1.pp.x + acc.x * delta_time + dpc.x,
                        y: p1.p.y - p1.pp.y + acc.y * delta_time + dpc.y,
                    };
                    let p = Point {
                        x: (p1.p.x + 1.0 + v.x).fract(),
                        y: (p1.p.y + 1.0 + v.y).fract(),
                    };
                    let mut pp = match p1.kind {
                        Kind::Firefly => {
                            let delta_firefly = 0.0005;
                            &(&p - &v)
                                * &Point {
                                    x: (1.0 - delta_firefly * 0.5
                                        + rand::random::<Float>() * delta_firefly),
                                    y: (1.0 - delta_firefly * 0.5
                                        + rand::random::<Float>() * delta_firefly),
                                }
                        }
                        _ => &p - &v,
                    };
                    let mx_speed = 0.001;
                    pp.x = pp.x.max(p.x - mx_speed).min(p.x + mx_speed);
                    pp.y = pp.y.max(p.y - mx_speed).min(p.y + mx_speed);
                    new_parts.insert(
                        *pid1,
                        Part {
                            p: p,
                            pp: pp,
                            d: p1.d,
                            m: p1.m,
                            kind: p1.kind,
                            trace_a: trace_a,
                            trace_b: trace_b,
                            trace_c: trace_c,
                            trace_d: trace_d,
                        },
                    );
                    new_datas.insert(*pid1, new_data);
                    let block = Block::new(&p, &self.blocks);
                    new_pids[block.x][block.y].push(*pid1);
                }
            }
        }
        self.pids = new_pids;
        self.parts = new_parts;
        self.datas = new_datas;
        self.parts_vec = self.parts.values().cloned().collect::<Vec<Part>>();
        self.step += 1;
    }
    pub fn parts_count(&self) -> usize {
        self.parts_vec.len()
    }
    pub fn parts_ptr(&self) -> *const Part {
        self.parts_vec.as_ptr()
    }
    pub fn get_step(&self) -> usize {
        self.step
    }
}
