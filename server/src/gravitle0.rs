use crate::data::Data;
use crate::maths::delta_position_wrap_around;
use crate::maths::distance_squared_wrap_around;
use crate::maths::normalize;
use crate::maths::dot;
use crate::part::Part;
use core::point::Point;
use crate::websocket;
use crate::websocket_async;
use crate::CellId;
use crate::Depth;
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
const DIAMETER_MIN: Float = 1.0 / 350.0 * 0.5;
const DIAMETER_MAX: Float = 1.0 / 350.0 * 0.5;
const WIDTH_X_HEIGHT: usize = WIDTH * HEIGHT;
const WEBSOCKET_ASYNC: bool = false;
const DELTA_TIME: Float = 1.0 / 60.0;
#[inline(always)]
pub fn cell_id(i: usize, j: usize) -> usize {
    i + j * WIDTH
}
#[inline(always)]
fn part_id_next(cid: CellId, depths: &[Depth]) -> usize {
    let k = depths[cid];
    assert!(k < MAX_DEPTH);
    part_id(cid, k)
}

#[inline(always)]
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
        })));
        data2s.push(Arc::new(RwLock::new(Data {
            parts: init_parts(),
            depths: vec![0; WIDTH_X_HEIGHT],
            links: vec![Vec::new();THREADS],
            step: 0,
            new_pids: vec![0; SIZE],
        })));
    }
    for _ in 0..TOTAL_COUNT/2 {
        let xx = rng.gen::<Float>();
        let yy = rng.gen::<Float>();
        let (thread_id1, pid1) = {
            let x = xx;
            let y = yy;
            let thread_id: usize = rng.gen_range(0..THREADS);
            let i: usize = (x * WIDTH as Float) as usize;
            let j: usize = (y * HEIGHT as Float) as usize;
            let cid = cell_id(i, j);
            let mut data1 = data1s[thread_id].write().unwrap();
            let pid = part_id_next(cid, &data1.depths);
            data1.depths[cid] += 1;
            data1.parts[pid].p.x = x;
            data1.parts[pid].p.y = y;
            let max_speed = 0.001 * 1.0;
            data1.parts[pid].pp.x = x + rng.gen::<Float>() * max_speed - max_speed*0.5;
            data1.parts[pid].pp.y = y + rng.gen::<Float>() * max_speed - max_speed*0.5;
            data1.parts[pid].d = (DIAMETER_MAX-DIAMETER_MIN)*rng.gen::<Float>() + DIAMETER_MIN;
            data1.parts[pid].m = rng.gen::<Float>() + 1.0;
            data1.new_pids[pid] = pid;
            (thread_id, pid)
        };
        let (thread_id2, pid2) = {
            let x = xx + DIAMETER_MAX * 1.2;
            let y = yy ;
            let thread_id: usize = rng.gen_range(0..THREADS);
            let i: usize = (x * WIDTH as Float) as usize;
            let j: usize = (y * HEIGHT as Float) as usize;
            let cid = cell_id(i, j);
            let mut data1 = data1s[thread_id].write().unwrap();
            let pid = part_id_next(cid, &data1.depths);
            data1.depths[cid] += 1;
            data1.parts[pid].p.x = x;
            data1.parts[pid].p.y = y;
            let max_speed = 0.00001 * 0.0;
            data1.parts[pid].pp.x = x + rng.gen::<Float>() * max_speed - max_speed*0.5;
            data1.parts[pid].pp.y = y + rng.gen::<Float>() * max_speed - max_speed*0.5;
            data1.parts[pid].d = (DIAMETER_MAX-DIAMETER_MIN)*rng.gen::<Float>() + DIAMETER_MIN;
            data1.parts[pid].m = rng.gen::<Float>() + 1.0;
            data1.new_pids[pid] = pid;
            (thread_id, pid)
        };
        // data1s[thread_id1].write().unwrap().links[thread_id2].push(Link {
        //     pid1: pid1,
        //     pid2: pid2,
        // });
        // data1s[thread_id2].write().unwrap().links[thread_id1].push(Link {
        //     pid1: pid2,
        //     pid2: pid1,
        // });
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

/*
    update links.pid
    Compute parts
    Compute links
    save parts to new pid
    update old_pid->new_pid
*/


fn compute_loop(d1s: &[Arc<RwLock<Data>>], d2s: &[Arc<RwLock<Data>>], thread_id: usize) {
    let mut tmp_parts = init_parts();
    let mut tmp_speeds: Vec<Point> = vec![Point{x:0.0, y:0.0}; SIZE];
    let mut tmp_pids: Vec<usize> = vec![0; SIZE];
    let mut tmp_count = 0;
    let mut tmp_depths: Vec<Depth> = vec![0; WIDTH_X_HEIGHT];

    let mut new_pids: Vec<usize> = vec![0; SIZE];
    // let mut tmp_links: Vec<Link> = vec![vec![Link{
    //     pid1: 0,
    //     pid2: 0,
    // }; SIZE];THREADS];

    let ends_count = 1000;
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
            loop {
                let mut b = true;
                {
                    let drs_: Vec<RwLockReadGuard<Data>> =
                        drs.iter().map(|x| x.read().unwrap()).collect();
                    let dr = drs[thread_id].read().unwrap();
                    for d in drs_.iter() {
                        if d.step < dr.step {
                            b = false;
                            break;
                        }
                    }
                }
                if b {
                    break;
                }
            }
        }


        // Update links
        // {
        //     let mut dw = dws[thread_id].write().unwrap();
        //     {
        //         let dr1 = drs[thread_id].read().unwrap();
        //         for (thread_id_2, links) in dr1.links.iter().enumerate() {
        //             dw.links[thread_id_2].resize(links.len(), Link{pid1:0, pid2:0});
        //             let dr2 = &drs[thread_id_2].read().unwrap();
        //             for i in 0..links.len() {
        //                 let link = &dr1.links[thread_id_2][i];
        //                 dw.links[thread_id_2][i].pid1 = dr1.new_pids[ link.pid1 ];
        //                 dw.links[thread_id_2][i].pid2 = dr2.new_pids[ link.pid2 ];
        //             }
        //         }
        //     }
        //     drs[thread_id].write().unwrap().links = dw.links.clone();
        // }


        let dw_step;
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

            // for (thread_id, links) in data_read.links.iter().enumerate() {
            //     let dr = &drs_[thread_id];
            //     for link in links {
            //         let p1 = data_read.parts[link.pid1];
            //         let p2 = dr.parts[link.pid2];
            //
            //         let distance_sqrd =
            //             distance_squared_wrap_around(&p1.p, &p2.p);
            //         let dpw = &delta_position_wrap_around(&p1.p, &p2.p);
            //         let diameter = (p1.d + p2.d) * 0.5;
            //         let v1x = p1.p.x - p1.pp.x;
            //         let v1y = p1.p.y - p1.pp.y;
            //         let v2x = p2.p.x - p2.pp.x;
            //         let v2y = p2.p.y - p2.pp.y;
            //         let dv = &Point {
            //             x: v1x - v2x,
            //             y: v1y - v2y,
            //         };
            //         let mass_factor = 2.0 * p1.m / (p1.m + p2.m);
            //         let dot_vp = dot(dv, dpw);
            //         let acceleration = Point {
            //             x: dpw.x * mass_factor * dot_vp / distance_sqrd,
            //             y: dpw.y * mass_factor * dot_vp / distance_sqrd
            //         };
            //         let colliding = distance_sqrd < diameter * diameter;
            //
            //         let strength = 10000.0;
            //         let link_force = normalize(dpw) * ((diameter * diameter - distance_sqrd) * strength);
            //         if (colliding) {
            //             tmp_speeds[link.pid1] += &(link_force / p1.m * DELTA_TIME + acceleration * 0.5);
            //         } else {
            //             tmp_speeds[link.pid1] += &(link_force / p1.m * DELTA_TIME );
            //         }
            //     }
            // }

            for i in 0..WIDTH {
                let i2s = [(i + WIDTH_LESS) % WIDTH, i, (i + WIDTH_MORE) % WIDTH];
                for j in 0..HEIGHT {
                    let j2s = [(j + HEIGHT_LESS) % HEIGHT, j, (j + HEIGHT_MORE) % HEIGHT];
                    let cid1 = cell_id(i, j);
                    for k in 0..data_read.depths[cid1] {
                        let pid1 = part_id(cid1, k);
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
                        new_pids[pid1] = pid_new;
                        tmp_count += 1;
                    }
                }
            }

            dw_step = data_read.step + 1;
        }
        let depths_clone = tmp_depths.clone();
        let links_clone = drs[thread_id].read().unwrap().links.clone();
        let new_pids_clone = new_pids.clone();
        {
            let mut dw = dws[thread_id].write().unwrap();
            for i in 0..tmp_count {
                let pid = tmp_pids[i];
                let part = tmp_parts[i];
                dw.parts[pid].p.x = part.p.x;
                dw.parts[pid].p.y = part.p.y;
                dw.parts[pid].d = part.d;
                dw.parts[pid].m = part.m;
                dw.parts[pid].pp.x = part.pp.x;
                dw.parts[pid].pp.y = part.pp.y;
            }
            dw.depths = depths_clone;
            dw.step = dw_step;
            dw.links = links_clone;
            dw.new_pids = new_pids_clone;
        }
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
