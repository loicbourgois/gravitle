use crate::data::Data;
use crate::maths::delta_position_wrap_around;
use crate::maths::distance_squared_wrap_around;
use crate::maths::normalize;
use crate::maths::dot;
use crate::part::Part;
use core::point::Point;
use crate::plan::Plan;
use crate::plan::Kind;
use crate::entity::add_entity;
use std::time::Duration;
use std::collections::HashSet;
use crate::websocket;
use crate::websocket_async;
use crate::plan::PartPlan;
use crate::CellId;
use crate::Depth;
use std::sync::RwLockWriteGuard;
use crate::Float;
use crate::link::Link;
use futures_util::{future, StreamExt, TryStreamExt};
use log::info;
use rand::Rng;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;
use std::thread;
use std::time::SystemTime;
use tokio::net::TcpListener as TokioTcpListener;
use tokio::net::TcpStream as TokioTcpStream;
const TOTAL_COUNT: usize = 100_000;
const THREADS: usize = 8;
pub const WIDTH: usize = 350;
const WIDTH_LESS: usize = WIDTH - 1;
const WIDTH_MORE: usize = WIDTH + 1;
pub const HEIGHT: usize = 350;
const HEIGHT_LESS: usize = HEIGHT - 1;
const HEIGHT_MORE: usize = HEIGHT + 1;
const MAX_DEPTH: Depth = 32;
const SIZE: usize = WIDTH * HEIGHT * MAX_DEPTH as usize;
pub const DIAMETER_MIN: Float = 1.0 / 350.0 * 0.5;
pub const DIAMETER_MAX: Float = 1.0 / 350.0 * 0.5;
const WIDTH_X_HEIGHT: usize = WIDTH * HEIGHT;
const WEBSOCKET_ASYNC: bool = false;
const DELTA_TIME: Float = 1.0 / 60.0;
const ends_count: usize = 100;
const LINK_STRENGTH: Float = 1000.0;
#[inline(always)]
pub fn cell_id(i: usize, j: usize) -> usize {
    i + j * WIDTH
}
// #[inline(always)]
pub fn part_id_next(cid: CellId, depths: &[Depth]) -> usize {
    let k = depths[cid];
    assert!(k < MAX_DEPTH);
    part_id(cid, k)
}

// #[inline(always)]
pub fn part_id(cid: CellId, depth: Depth) -> usize {
    cid * MAX_DEPTH as usize + depth as usize
}

pub fn init_parts() -> Vec<Part> {
    vec![
        Part {
            p: Point { x: 0.0, y: 0.0 },
            pp: Point { x: 0.0, y: 0.0 },
            d: 0.0,
            m: 0.0,
        };
        SIZE
    ]
}

pub async fn start() {
    let mut data1s: Vec<Arc<RwLock<Data>>> = Vec::new();
    let mut data2s: Vec<Arc<RwLock<Data>>> = Vec::new();
    let timer_init = SystemTime::now();
    let mut rng = rand::thread_rng();
    for _ in 0..THREADS {
        data1s.push(Arc::new(RwLock::new(Data {
            parts: init_parts(),
            depths: vec![0; WIDTH_X_HEIGHT],
            links: vec![Vec::new();THREADS],
            step: 0,
            new_pids: vec![0; SIZE],
            parts_to_remove: HashSet::new(),
        })));
        data2s.push(Arc::new(RwLock::new(Data {
            parts: init_parts(),
            depths: vec![0; WIDTH_X_HEIGHT],
            links: vec![Vec::new();THREADS],
            step: 0,
            new_pids: vec![0; SIZE],
            parts_to_remove: HashSet::new(),
        })));
    }
    let plan: Plan = Plan {
        kinds: [Kind::Metal, Kind::Metal],
        part_plans: vec![
            PartPlan{a:0,b:1,k:Kind::Metal},
            PartPlan{a:1,b:0,k:Kind::Metal},
            PartPlan{a:1,b:3,k:Kind::Metal},
            PartPlan{a:3,b:0,k:Kind::Metal},
            PartPlan{a:1,b:4,k:Kind::Metal},
            PartPlan{a:5,b:0,k:Kind::Metal},
            PartPlan{a:1,b:6,k:Kind::Metal},
            PartPlan{a:7,b:0,k:Kind::Metal},
            PartPlan{a:6,b:4,k:Kind::Metal},
            PartPlan{a:5,b:7,k:Kind::Metal},
        ],
    };
    for _ in 0..TOTAL_COUNT/12 {
        let position = Point {
            x: rng.gen::<Float>(),
            y: rng.gen::<Float>(),
        };
        let rotation = rng.gen::<Float>();
        let thread_id = rng.gen_range(0..THREADS);
        add_entity(&mut data1s, &plan, &position, rotation, thread_id);
    }
    println!("  init: {:?}", timer_init.elapsed().unwrap());
    let mut handles = Vec::new();
    let timer_steps = SystemTime::now();
    for i in 0..THREADS {
        let d1s = data1s.clone();
        let d2s = data2s.clone();
        handles.push(thread::spawn(move || {
            compute_loop(&d1s, &d2s, i);
        }));
    }
    let senders: websocket::Senders = Arc::new(Mutex::new(HashMap::new()));
    if WEBSOCKET_ASYNC {
        websocket_async::serve_async(&senders).await;
    } else {
        websocket::serve(&senders);
        websocket::send(&websocket::SendArgs {
            senders: &senders,
            datas: &data1s,
        });
    }
    for handle in handles {
        handle.join().unwrap();
    }
}



fn compute_loop(d1s: &[Arc<RwLock<Data>>], d2s: &[Arc<RwLock<Data>>], thread_id: usize) {
    let mut tmp_parts = init_parts();
    let mut tmp_speeds: Vec<Point> = vec![Point{x:0.0, y:0.0}; SIZE];
    let mut tmp_pids: Vec<usize> = vec![0; SIZE];
    let mut tmp_count = 0;
    let mut tmp_depths: Vec<Depth> = vec![0; WIDTH_X_HEIGHT];
    let mut old_pids: Vec<usize> = vec![0; SIZE];
    let mut ends = vec![SystemTime::now(); ends_count];
    let mut step = 0;
    let start = SystemTime::now();
    loop {
        let mut collision_tests = 0;
        tmp_count = 0;
        for tmp_depth in tmp_depths.iter_mut().take(WIDTH_X_HEIGHT) {
            *tmp_depth = 0;
        }
        let (drs, dws) = {
            if step % 2 == 0 {
                (d1s, d2s)
            } else {
                (d2s, d1s)
            }
        };

        {
            let mut min = 0;
            loop {
                let mut b = true;
                {
                    let drs_: Vec<RwLockReadGuard<Data>> =
                        drs.iter().map(|x| x.read().unwrap()).collect();
                    let dr = drs[thread_id].read().unwrap();
                    for i  in min..drs_.len() {
                        let d = &drs_[i];
                        if d.step < dr.step {
                            b = false;
                            break;
                        }
                        min = i;
                    }
                }
                if b {
                    break;
                }
            }
        }


        let dw_step;
        let mut links_to_remove: Vec<Vec<usize>> = vec![Vec::new();THREADS];
        {
            let data_read = drs[thread_id].read().unwrap();
            let drs_: Vec<RwLockReadGuard<Data>> = drs.iter().map(|x| x.read().unwrap()).collect();
            for i in 0..WIDTH {
                let i2s = [(i + WIDTH_LESS) % WIDTH, i, (i + WIDTH_MORE) % WIDTH];
                for j in 0..HEIGHT {
                    let j2s = [(j + HEIGHT_LESS) % HEIGHT, j, (j + HEIGHT_MORE) % HEIGHT];
                    let cid1 = cell_id(i, j);
                    for k in 0..data_read.depths[cid1] {
                        let pid1 = part_id(cid1, k);

                        if data_read.parts_to_remove.contains(&pid1) {
                            continue;
                        } else {
                            // ok
                        }

                        let p1 = data_read.parts[pid1];
                        let mut d_collision = Point {
                            x: 0.0,
                            y: 0.0
                        };
                        let mut forces = Point {
                            x: 0.0,
                            y: 0.0
                        };
                        let mut collisions = 0;
                        for i2 in i2s {
                            for j2 in j2s {
                                let cid2 = cell_id(i2, j2);
                                for (tid2, dr2) in drs_.iter().enumerate().take(THREADS) {
                                    for k2 in 0..dr2.depths[cid2] {
                                        let pid2 = part_id(cid2, k2);

                                        if dr2.parts_to_remove.contains(&pid2) {
                                            continue;
                                        } else {
                                            // ok
                                        }

                                        if pid1 != pid2 || tid2 != thread_id {
                                            collision_tests += 1;
                                            let p2 = dr2.parts[pid2];
                                            let distance_sqrd =
                                                distance_squared_wrap_around(&p1.p, &p2.p);
                                            let dpw = &delta_position_wrap_around(&p1.p, &p2.p);
                                            let diameter = (p1.d + p2.d) * 0.5;
                                            let colliding = distance_sqrd < diameter * diameter;
                                            if colliding {
                                                // https://en.wikipedia.org/wiki/Elastic_collision#Two-dimensional_collision_with_two_moving_objects
                                                let v1x = p1.p.x - p1.pp.x;
                                                let v1y = p1.p.y - p1.pp.y;
                                                let v2x = p2.p.x - p2.pp.x;
                                                let v2y = p2.p.y - p2.pp.y;
                                                let dv = &Point {
                                                    x: v1x - v2x,
                                                    y: v1y - v2y,
                                                };
                                                let mass_factor = 2.0 * p1.m / (p1.m + p2.m);
                                                let dot_vp = dot(dv, dpw);
                                                let acceleration = Point {
                                                    x: dpw.x * mass_factor * dot_vp / distance_sqrd,
                                                    y: dpw.y * mass_factor * dot_vp / distance_sqrd
                                                };
                                                d_collision -= acceleration;
                                                collisions += 1;
                                            } else {
                                                // pass
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        let acceleration = forces / p1.m;
                        tmp_speeds[pid1] = p1.p - p1.pp + acceleration * DELTA_TIME + d_collision;
                    }
                }
            }

            for (thid2, links) in data_read.links.iter().enumerate() {
                let dr = &drs_[thid2];
                for (link_id, link) in links.iter().enumerate() {

                    if data_read.parts_to_remove.contains(&link.pid1) || dr.parts_to_remove.contains(&link.pid2) {
                        links_to_remove[thid2].push(link_id);
                        continue;
                    } else {
                        // ok
                    }

                    let p1 = data_read.parts[link.pid1];
                    let p2 = dr.parts[link.pid2];
                    let distance_sqrd =
                        distance_squared_wrap_around(&p1.p, &p2.p);
                    let dpw = &delta_position_wrap_around(&p1.p, &p2.p);
                    let diameter = (p1.d + p2.d) * 0.5;
                    let v1x = p1.p.x - p1.pp.x;
                    let v1y = p1.p.y - p1.pp.y;
                    let v2x = p2.p.x - p2.pp.x;
                    let v2y = p2.p.y - p2.pp.y;
                    let dv = &Point {
                        x: v1x - v2x,
                        y: v1y - v2y,
                    };
                    let mass_factor = 2.0 * p1.m / (p1.m + p2.m);
                    let dot_vp = dot(dv, dpw);
                    let acceleration = Point {
                        x: dpw.x * mass_factor * dot_vp / distance_sqrd,
                        y: dpw.y * mass_factor * dot_vp / distance_sqrd
                    };
                    let colliding = distance_sqrd < diameter * diameter;
                    let link_force = normalize(dpw) * ((diameter * diameter - distance_sqrd) * LINK_STRENGTH);
                    if (colliding) {
                        tmp_speeds[link.pid1] += &(link_force / p1.m * DELTA_TIME + acceleration * 0.5);
                    } else {
                        tmp_speeds[link.pid1] += &(link_force / p1.m * DELTA_TIME );
                    }
                }
            }

            for i in 0..WIDTH {
                let i2s = [(i + WIDTH_LESS) % WIDTH, i, (i + WIDTH_MORE) % WIDTH];
                for j in 0..HEIGHT {
                    let j2s = [(j + HEIGHT_LESS) % HEIGHT, j, (j + HEIGHT_MORE) % HEIGHT];
                    let cid1 = cell_id(i, j);
                    for k in 0..data_read.depths[cid1] {
                        let pid1 = part_id(cid1, k);

                        if data_read.parts_to_remove.contains(&pid1) {
                            continue;
                        } else {
                            // ok
                        }

                        let p1 = data_read.parts[pid1];
                        let max_speed = 0.0001;
                        tmp_speeds[pid1].x = tmp_speeds[pid1].x.max(-max_speed).min(max_speed);
                        tmp_speeds[pid1].y = tmp_speeds[pid1].y.max(-max_speed).min(max_speed);
                        let x = (p1.p.x + tmp_speeds[pid1].x + 1.0).fract();
                        let y = (p1.p.y + tmp_speeds[pid1].y + 1.0).fract();
                        let x_ = x - tmp_speeds[pid1].x;
                        let y_ = y - tmp_speeds[pid1].y;
                        let i_new: usize = (x * WIDTH as Float) as usize;
                        let j_new: usize = (y * HEIGHT as Float) as usize;
                        let cid_new: CellId = cell_id(i_new, j_new);
                        let pid_new = part_id_next(cid_new, &tmp_depths);
                        tmp_depths[cid_new] += 1;
                        tmp_parts[tmp_count].p.x = x;
                        tmp_parts[tmp_count].p.y = y;
                        tmp_parts[tmp_count].pp.x = x_;
                        tmp_parts[tmp_count].pp.y = y_;
                        tmp_parts[tmp_count].d = p1.d;
                        tmp_parts[tmp_count].m = p1.m;
                        tmp_pids[tmp_count] = pid_new;
                        // new_pids[tmp_count] = pid_new;
                        old_pids[tmp_count] = pid1;
                        tmp_count += 1;
                    }
                }
            }

            dw_step = data_read.step + 1;
        }
        let depths_clone = tmp_depths.clone();
        let mut links_clone = drs[thread_id].read().unwrap().links.clone();
        for i in 0..THREADS {
            let l = links_to_remove[i].len();
            //println!("{}", links_clone.len());
            for j in (0..l).rev() {
                links_clone[i].remove(links_to_remove[i][j]);
            }
        }

        let links_clone_2 = links_clone.clone();
        {
            let mut dw = dws[thread_id].write().unwrap();
            dw.parts_to_remove.clear();
            for i in 0..tmp_count {
                let pid = tmp_pids[i];
                let part = tmp_parts[i];
                dw.parts[pid].p.x = part.p.x;
                dw.parts[pid].p.y = part.p.y;
                dw.parts[pid].d = part.d;
                dw.parts[pid].m = part.m;
                dw.parts[pid].pp.x = part.pp.x;
                dw.parts[pid].pp.y = part.pp.y;
                dw.new_pids[old_pids[i]] = pid;
            }
            dw.depths = depths_clone;
            if thread_id != 0 {
                dw.step = dw_step;
            }
            dw.links = links_clone;
        }
        if thread_id == 0 {
            // Wait for all writes to be done
            {
                let mut min = 0;
                loop {
                    let mut b = true;
                    {
                        let dws_: Vec<RwLockReadGuard<Data>> =
                            dws.iter().map(|x| x.read().unwrap()).collect();
                        let dw = dws[thread_id].read().unwrap();
                        for thread_id_  in min..dws_.len() {
                            let d = &dws_[thread_id_];
                            if d.step < dw_step && thread_id_ != thread_id {
                                b = false;
                                break;
                            }
                            min = thread_id_;
                        }
                    }
                    if b {
                        break;
                    }
                }
            }
            // Update links
            {
                let mut ds: Vec<RwLockWriteGuard<Data>> =
                    dws.iter().map(|x| x.write().unwrap()).collect();
                for thid1 in 0..THREADS {
                    for thid2 in 0..THREADS {
                        let l = ds[thid1].links[thid2].len();
                        for i in 0..l {
                            ds[thid1].links[thid2][i].pid1 = ds[thid1].new_pids[ ds[thid1].links[thid2][i].pid1 ];
                            ds[thid1].links[thid2][i].pid2 = ds[thid2].new_pids[ ds[thid1].links[thid2][i].pid2 ];
                        }
                    }
                }
            }
            dws[thread_id].write().unwrap().step = dw_step;
        }
        // Wait for all writes to be done
        {
            let mut min = 0;
            loop {
                let mut b = true;
                {
                    let dws_: Vec<RwLockReadGuard<Data>> =
                        dws.iter().map(|x| x.read().unwrap()).collect();
                    let dw = dws[thread_id].read().unwrap();
                    for i  in min..dws_.len() {
                        let d = &dws_[i];
                        if d.step < dw.step {
                            b = false;
                            break;
                        }
                        min = i;
                    }
                }
                if b {
                    break;
                }
            }
        }
        //
        ends[dw_step % ends_count] = SystemTime::now();
        if dw_step % 100 == 0 && thread_id == 0 {
            let duration = ends[(dw_step + 1) % ends_count].elapsed().unwrap() / ends_count as u32;
            let duration_ = ends[(dw_step + 1) % ends_count]
                .elapsed()
                .unwrap()
                .as_secs_f32()
                / ends_count as Float;
            let cps = 1.0 / duration_;
            let mpps = (0.000001 / duration_) * (TOTAL_COUNT) as Float;
            let global_duration = start.elapsed().unwrap();
            let global_cps = dw_step as Float / start.elapsed().unwrap().as_secs_f32();
            let global_mpps = global_cps * (TOTAL_COUNT as Float) * 0.000001;
            println!(
                "\
Thread #{}
    step: {}
    current:
        duration: {:?}
        cps:      {}
        pps:      {:.2} MP/s
    average:
        duration: {:?}
        cps:      {}
        pps:      {:.2} MP/s",
                thread_id, dw_step, duration, cps, mpps, global_duration, global_cps, global_mpps
            );
        }
        step = dw_step;
    }
}