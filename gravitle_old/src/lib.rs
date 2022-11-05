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
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
// use std::{
//     time::{Duration,SystemTime}
// };
use utils::log;
use uuid::Uuid;
use wasm_bindgen::prelude::wasm_bindgen;
const BASE_CAPACITY: usize = 2;
const DIAMETER: Float = 0.004;
const MASS: Float = 1.0;
const LINK_STRENGTH: Float = 1000.0;
const NEW_EGG_TRIGGER: Float = 1.2;
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
    Core = 7,
    Egg = 8,
}
#[wasm_bindgen]
#[repr(u32)]
#[derive(Clone, Copy, Debug, Hash, std::cmp::Eq, std::cmp::PartialEq)]
pub enum DataKind {
    Activity = 1,
    // Water = 2,
    Energy = 3,
    Bias = 4,
}
//
// pub enum Data {
//     Activity(Float),
//     // Water = 2,
//     Energy(Float),
//     Weights(Vec<Float>),
//     Links(Vec<u128>),
// }

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
    direction: Point,
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
    neuro_links: HashMap<u128, Vec<(u128, Float)>>,
    total_energy: Float,
    counters: Vec<Counter>,
}

const COUNTER_GLOBAL: usize = 0;
const COUNTER_COLLISION: usize = 1;
const COUNTER_ACTIVITY: usize = 2;
const COUNTER_LINKED: usize = 3;
const COUNTERS: usize = 4;
const COUNTER_SIZE: usize = 10;
//extern crate web_sys;
use web_sys;

fn now() -> f64 {
    web_sys::window()
        .expect("should have a Window")
        .performance()
        .expect("should have a Performance")
        .now()
}

struct Counter {
    values: Vec<f64>,
    size: usize,
    value: f64,
    start: f64,
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
        let mut counters = Vec::new();
        for i in 0..COUNTERS {
            counters.push(Counter {
                value: 0.0,
                values: Vec::new(),
                size: COUNTER_SIZE,
                start: now(),
            });
        }
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
            total_energy: 0.0,
            counters: counters,
            neuro_links: HashMap::new(),
        }
    }

    pub fn add_part(&mut self, kind: Kind, p: &Point, v: &Point) {
        self.add_part_(kind, p, v);
    }

    fn add_part_(&mut self, kind: Kind, p: &Point, v: &Point) -> u128 {
        self.add_part__(kind, p, v, 0.1)
    }

    fn add_part__(&mut self, kind: Kind, p: &Point, v: &Point, diameter: Float) -> u128 {
        let block = Block::new(&p, &self.blocks);
        let pid: u128 = Uuid::new_v4().as_u128();
        self.parts.insert(
            pid,
            Part {
                p: *p,
                pp: p - v,
                d: DIAMETER * diameter,
                m: MASS,
                kind: kind,
                trace_a: 0.0,
                trace_b: 0.0,
                trace_c: 0.0,
                trace_d: 0.0,
                direction: Point { x: 0.0, y: 0.0 },
            },
        );
        let mut data_map = HashMap::new();
        data_map.insert(DataKind::Energy, 1.0);
        match kind {
            Kind::Turbo => {
                data_map.insert(DataKind::Activity, 0.0);
            }
            Kind::Neuron => {
                data_map.insert(DataKind::Activity, 0.0);
                data_map.insert(DataKind::Bias, rand::random::<Float>() * 2.0 - 1.0);
            }
            _ => {}
        }
        self.datas.insert(pid, data_map);
        self.pids[block.x][block.y].push(pid);
        pid
    }

    fn add_link(&mut self, pid1: &u128, pid2: &u128, strengh: Float) {
        match self.links.get_mut(pid1) {
            Some(a) => match a.get_mut(pid2) {
                Some(b) => {
                    log("should not happen");
                }
                None => {
                    a.insert(*pid2, strengh);
                }
            },
            None => {
                let mut hashmap = HashMap::new();
                hashmap.insert(*pid2, strengh);
                self.links.insert(*pid1, hashmap);
            }
        }
        match self.links.get_mut(pid2) {
            Some(a) => match a.get_mut(pid1) {
                Some(b) => {
                    log("should not happen");
                }
                None => {
                    a.insert(*pid1, strengh);
                }
            },
            None => {
                let mut hashmap = HashMap::new();
                hashmap.insert(*pid1, strengh);
                self.links.insert(*pid2, hashmap);
            }
        }
        let p1 = self.parts.get(pid1).unwrap();
        let p2 = self.parts.get(pid2).unwrap();
        let d1 = self.datas.get(pid1).unwrap();
        let d2 = self.datas.get(pid2).unwrap();
        match (
            p1.kind,
            d1.get(&DataKind::Activity),
            d2.get(&DataKind::Activity),
        ) {
            (Kind::Neuron, Some(a1), Some(a2)) => match self.neuro_links.get_mut(pid1) {
                Some(a) => {
                    a.push((*pid2, rand::random::<Float>() * 2.0 - 1.0));
                }
                None => {
                    self.neuro_links
                        .insert(*pid1, vec![(*pid2, rand::random::<Float>() * 2.0 - 1.0)]);
                }
            },
            _ => {}
        }
        match (
            p2.kind,
            d1.get(&DataKind::Activity),
            d2.get(&DataKind::Activity),
        ) {
            (Kind::Neuron, Some(a1), Some(a2)) => match self.neuro_links.get_mut(pid2) {
                Some(a) => {
                    a.push((*pid1, rand::random::<Float>() * 2.0 - 1.0));
                }
                None => {
                    self.neuro_links
                        .insert(*pid2, vec![(*pid1, rand::random::<Float>() * 2.0 - 1.0)]);
                }
            },
            _ => {}
        }
    }

    pub fn add_entity(&mut self, p: &Plan, c: &Point) {
        self.add_entity_(p, c, 0.1);
    }

    pub fn add_entity_full(&mut self, p: &Plan, c: &Point) {
        self.add_entity_(p, c, 1.0);
    }

    pub fn add_entity_(&mut self, p: &Plan, c: &Point, diameter: Float) {
        let v = Point { x: 0.0, y: 0.0 };
        let p1 = c - &Point {
            x: DIAMETER * 0.5 * diameter,
            y: 0.0,
        };
        let p2 = c + &Point {
            x: DIAMETER * 0.5 * diameter,
            y: 0.0,
        };
        let mut coords = Vec::new();
        let mut pids = Vec::new();
        coords.push(p1);
        coords.push(p2);
        pids.push(self.add_part__(p.k1, &p1, &v, diameter));
        pids.push(self.add_part__(p.k2, &p2, &v, diameter));
        self.add_link(&pids[0], &pids[1], LINK_STRENGTH);
        for part in p.part_plans_().iter() {
            let pos = p_coords(&coords[part.a], &coords[part.b]);
            let pid1 = self.add_part__(part.k, &pos, &v, diameter);
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
        self.counters[COUNTER_GLOBAL].start = now();
        let mut new_parts: HashMap<u128, Part> = HashMap::new();
        let mut new_datas: HashMap<u128, HashMap<DataKind, Float>> = HashMap::new();
        let mut new_pids: Vec<Vec<Vec<u128>>> =
            vec![vec![Vec::with_capacity(BASE_CAPACITY); self.blocks.h]; self.blocks.w];
        let mut new_eggs: HashSet<u128> = HashSet::new();
        let mut to_deletes: HashSet<u128> = HashSet::new();
        let mut server_total_energy = 0.0;
        let mut counter_collision = 0.0;
        let mut counter_linked = 0.0;
        let mut counter_activity = 0.0;
        for i in 0..self.blocks.w {
            for j in 0..self.blocks.h {
                for pid1 in self.pids[i][j].iter() {
                    let p1 = self.parts.get(pid1).unwrap();
                    let d1 = self.datas.get(pid1).unwrap();
                    let mut dpc = Point { x: 0.0, y: 0.0 };
                    let mut force = Point { x: 0.0, y: 0.0 };
                    let mut direction_sum = Point { x: 0.0, y: 0.0 };
                    let mut activity_sum = 0.0;
                    let mut activity_count = 0.0;
                    let mut energy_to_add = 0.0;
                    let i2min = (i + self.blocks.w - 1) % self.blocks.w;
                    let i2max = (i + self.blocks.w + 1) % self.blocks.w;
                    let counter_collision_start = now();
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
                                    let counter_linked_start = now();
                                    if collision_response.linked {
                                        direction_sum += &delta_position_wrap_around(&p1.p, &p2.p);
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
                                        match (d1.get(&DataKind::Energy), d2.get(&DataKind::Energy))
                                        {
                                            (Some(e1), Some(e2)) => {
                                                energy_to_add += (e2 - e1) * 0.1;
                                            }
                                            _ => {}
                                        }
                                    } else {
                                        match (
                                            collision_response.colliding,
                                            d1.get(&DataKind::Energy),
                                            d2.get(&DataKind::Energy),
                                            p1.kind,
                                            p2.kind,
                                        ) {
                                            (true, Some(_), Some(_), Kind::Mouth, Kind::Mouth) => {}
                                            (true, Some(_), Some(e2), Kind::Mouth, _) => {
                                                energy_to_add += e2 * 0.5;
                                            }
                                            (true, Some(e1), Some(_), _, Kind::Mouth) => {
                                                energy_to_add -= e1 * 0.5;
                                            }
                                            _ => {}
                                        }
                                    }
                                    counter_linked += now() - counter_linked_start;
                                }
                            }
                        }
                    }
                    counter_collision += now() - counter_collision_start;
                    // Energy + Creation
                    let mut to_delete = false;
                    let mut new_data = HashMap::new();
                    match d1.get(&DataKind::Energy) {
                        Some(energy) => {
                            let total_energy = energy + energy_to_add;
                            let new_energy = match p1.kind {
                                Kind::Core => {
                                    if total_energy >= NEW_EGG_TRIGGER {
                                        new_eggs.insert(*pid1);
                                        total_energy * 0.5
                                    } else {
                                        total_energy
                                    }
                                }
                                _ => total_energy,
                            };
                            if (new_energy <= 0.0) {
                                to_deletes.insert(*pid1);
                                to_delete = true;
                            }
                            new_data.insert(DataKind::Energy, new_energy);
                            server_total_energy += new_energy
                        }
                        None => {}
                    };
                    // Activity
                    let counter_activity_start = now();
                    match p1.kind {
                        Kind::Neuron => {
                            let bias = *d1.get(&DataKind::Bias).unwrap();
                            let mut activity_sum_ = bias;
                            let mut weights_sum = 1.0;
                            match self.neuro_links.get(pid1) {
                                Some(links) => {
                                    for x in links {
                                        match self.datas.get(&x.0) {
                                            Some(data) => {
                                                activity_sum_ +=
                                                    data.get(&DataKind::Activity).unwrap() * x.1;
                                                weights_sum += x.1.abs();
                                            }
                                            None => {
                                                // ok
                                            }
                                        }
                                    }
                                }
                                None => {
                                    log("should not happen #3");
                                }
                            }
                            new_data.insert(DataKind::Activity, activity_sum_ / weights_sum);
                            new_data.insert(DataKind::Bias, bias);
                        }
                        Kind::Turbo => {
                            new_data.insert(DataKind::Activity, activity_sum / activity_count);
                        }
                        _ => {}
                    };
                    counter_activity += now() - counter_activity_start;
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
                        Kind::Core => {
                            trace_a = *d1.get(&DataKind::Energy).unwrap();
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
                    let direction = normalize(&direction_sum);
                    match p1.kind {
                        Kind::Turbo => {
                            let strength =
                                0.001 * d1.get(&DataKind::Activity).unwrap().max(0.0).min(1.0);
                            if direction.x.is_finite() && direction.y.is_finite() {
                                force.x -= direction.x * strength;
                                force.y -= direction.y * strength;
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
                    if !to_delete {
                        new_parts.insert(
                            *pid1,
                            Part {
                                p: p,
                                pp: pp,
                                d: (p1.d + DIAMETER * 0.005).max(DIAMETER * 0.1).min(DIAMETER),
                                m: p1.m,
                                kind: p1.kind,
                                trace_a: trace_a,
                                trace_b: trace_b,
                                trace_c: trace_c,
                                trace_d: trace_d,
                                direction: direction,
                            },
                        );
                        new_datas.insert(*pid1, new_data);
                        let block = Block::new(&p, &self.blocks);
                        new_pids[block.x][block.y].push(*pid1);
                    }
                }
            }
        }

        self.pids = new_pids;
        self.parts = new_parts;
        self.datas = new_datas;
        self.parts_vec = self.parts.values().cloned().collect::<Vec<Part>>();
        self.step += 1;
        for parent_core_pid in new_eggs.iter() {
            let core_part = self.parts.get(parent_core_pid).unwrap();
            let pos = core_part.p + core_part.direction * core_part.d * 0.5;
            let v = Point { x: 0.0, y: 0.0 };
            let pid = self.add_part_(Kind::Egg, &pos, &v);
            let energy = *self
                .datas
                .get(parent_core_pid)
                .unwrap()
                .get(&DataKind::Energy)
                .unwrap();
            self.datas
                .get_mut(&pid)
                .unwrap()
                .insert(DataKind::Energy, energy);
            server_total_energy += energy;
        }
        for pid1 in to_deletes.iter() {
            match self.links.remove(pid1) {
                Some(linked_pids) => {
                    for pid2 in linked_pids.keys() {
                        match self.links.get_mut(pid2) {
                            Some(a) => {
                                a.remove(pid1);
                            }
                            None => {}
                        }
                    }
                }
                None => {}
            }
            match self.neuro_links.remove(pid1) {
                Some(linked_pids) => {}
                None => {}
            }
        }
        self.total_energy = server_total_energy;
        if self.step % 10 == 0 {
            for k in 0..COUNTERS {
                if self.counters[k].values.len() > self.counters[k].size {
                    self.counters[k].values.drain(0..1);
                }
                self.counters[k].value = 0.0;
                let mut v_sum = 0.0;
                for v in &self.counters[k].values {
                    v_sum += v;
                }
                self.counters[k].value = v_sum / (self.counters[k].values.len() as f64);
            }
            let duration = now() - self.counters[COUNTER_GLOBAL].start;
            self.counters[COUNTER_GLOBAL].values.push(duration);
            self.counters[COUNTER_COLLISION]
                .values
                .push(counter_collision);
            self.counters[COUNTER_ACTIVITY]
                .values
                .push(counter_activity);
            self.counters[COUNTER_LINKED].values.push(counter_linked);
        }
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
    pub fn get_total_energy(&self) -> Float {
        self.total_energy
    }
    pub fn get_counter_value(&self, i: usize) -> f64 {
        self.counters[i].value
    }
}
