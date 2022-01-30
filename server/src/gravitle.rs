use crate::data::Data;
use crate::data::EntityToAdd;
use crate::entity::add_entity_2;
use crate::maths::delta_position_wrap_around;
use crate::maths::distance_squared_wrap_around;
use crate::maths::dot;
use crate::maths::normalize;
use crate::part::Part;
use crate::plan::PartPlan;
use crate::plan::Plan;
use crate::websocket;
use crate::websocket_async;
use crate::CellId;
use crate::Depth;
use crate::Float;
use chrono::Utc;
use core::part::Kind;
use core::point::Point;
use rand::Rng;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;
use std::thread;
use std::time::SystemTime;
const TOTAL_COUNT: usize = 50_000;
const THREADS: usize = 8;
const DIMENSION: usize = 450;
pub const WIDTH: usize = DIMENSION;
const WIDTH_LESS: usize = WIDTH - 1;
const WIDTH_MORE: usize = WIDTH + 1;
pub const HEIGHT: usize = DIMENSION;
const HEIGHT_LESS: usize = HEIGHT - 1;
const HEIGHT_MORE: usize = HEIGHT + 1;
const MAX_DEPTH: Depth = 64;
const SIZE: usize = WIDTH * HEIGHT * MAX_DEPTH as usize;
pub const DIAMETER_NEW: Float = 0.05 / (DIMENSION as Float);
pub const DIAMETER_MIN: Float = 0.5 / (DIMENSION as Float);
pub const DIAMETER_MAX: Float = 1.0 / (DIMENSION as Float);
const GROWTH: Float = DIAMETER_MIN * 0.001;
const WIDTH_X_HEIGHT: usize = WIDTH * HEIGHT;
const WEBSOCKET_ASYNC: bool = false;
const DELTA_TIME: Float = 1.0 / 60.0;
const DATA_POINTS_COUNT: usize = 100;
const LINK_STRENGTH: Float = 1000.0;
const ENERGY_TRANSFER: Float = 0.1;
const ENERGY_TRANSFER_EAT: Float = 0.01;
#[inline(always)]
pub fn cell_id(i: usize, j: usize) -> usize {
    i + j * WIDTH
}

pub fn part_id_next(cid: CellId, depths: &[Depth]) -> usize {
    let k = depths[cid];
    assert!(k < MAX_DEPTH);
    part_id(cid, k)
}

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
            kind: Kind::Invalid,
            energy: 0.0,
        };
        SIZE
    ]
}

pub fn get_plan() -> Plan {
    Plan {
        kinds: [Kind::Metal, Kind::Metal],
        part_plans: vec![
            PartPlan {
                a: 0,
                b: 1,
                k: Kind::Mouth,
            },
            PartPlan {
                a: 1,
                b: 0,
                k: Kind::Core,
            },
            PartPlan {
                a: 1,
                b: 3,
                k: Kind::Metal,
            },
            PartPlan {
                a: 3,
                b: 0,
                k: Kind::Metal,
            },
            PartPlan {
                a: 1,
                b: 4,
                k: Kind::Metal,
            },
            PartPlan {
                a: 5,
                b: 0,
                k: Kind::Metal,
            },
            PartPlan {
                a: 1,
                b: 6,
                k: Kind::Mouth,
            },
            PartPlan {
                a: 7,
                b: 0,
                k: Kind::Mouth,
            },
            PartPlan {
                a: 6,
                b: 4,
                k: Kind::Turbo,
            },
            PartPlan {
                a: 5,
                b: 7,
                k: Kind::Turbo,
            },
            //
            //
            PartPlan {
                a: 6,
                b: 10,
                k: Kind::Metal,
            },
            PartPlan {
                a: 12,
                b: 10,
                k: Kind::Metal,
            },
            PartPlan {
                a: 11,
                b: 7,
                k: Kind::Metal,
            },
            PartPlan {
                a: 11,
                b: 14,
                k: Kind::Metal,
            },
        ],
    }
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
            links: vec![Vec::new(); THREADS],
            step: 0,
            new_pids: vec![0; SIZE],
            parts_to_remove: HashSet::new(),
            entities_to_add: Vec::new(),
        })));
        data2s.push(Arc::new(RwLock::new(Data {
            parts: init_parts(),
            depths: vec![0; WIDTH_X_HEIGHT],
            links: vec![Vec::new(); THREADS],
            step: 0,
            new_pids: vec![0; SIZE],
            parts_to_remove: HashSet::new(),
            entities_to_add: Vec::new(),
        })));
    }
    let plan = get_plan();
    let part_plans_count: usize = plan.part_plans.len() + 2;
    {
        let mut datas: Vec<RwLockWriteGuard<Data>> =
            data1s.iter().map(|x| x.write().unwrap()).collect();
        for _ in 0..TOTAL_COUNT / part_plans_count {
            let position = Point {
                x: rng.gen::<Float>(),
                y: rng.gen::<Float>(),
            };
            let rotation = rng.gen::<Float>();
            let thread_id = rng.gen_range(0..THREADS);
            add_entity_2(
                &mut datas,
                &plan,
                &position,
                rotation,
                thread_id,
                (part_plans_count as Float) * 0.8 ,
            );
        }
    }
    println!("  init: {:?}", timer_init.elapsed().unwrap());
    let mut handles = Vec::new();
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

#[derive(Clone)]
struct Direction {
    neighbour_count: Float,
    direction: Point,
}

fn compute_loop(d1s: &[Arc<RwLock<Data>>], d2s: &[Arc<RwLock<Data>>], thread_id: usize) {
    let mut tmp_parts = init_parts();
    let mut tmp_speeds: Vec<Point> = vec![Point { x: 0.0, y: 0.0 }; SIZE];
    let mut tmp_pids: Vec<usize> = vec![0; SIZE];
    let mut tmp_energies_delta: Vec<Float> = vec![0.0; SIZE];
    let mut part_ok: Vec<bool> = vec![true; SIZE];
    let mut tmp_count;
    let mut tmp_depths: Vec<Depth> = vec![0; WIDTH_X_HEIGHT];

    let mut tmp_directions: Vec<Direction> = vec![
        Direction {
            neighbour_count: 0.0,
            direction: Point { x: 0.0, y: 0.0 }
        };
        SIZE
    ];

    let mut old_pids: Vec<usize> = vec![0; SIZE];
    let mut ends = vec![SystemTime::now(); DATA_POINTS_COUNT];
    let mut step = 0;
    let start = SystemTime::now();
    loop {
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
                    for (i, d) in drs_.iter().enumerate().skip(min) {
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
        let mut collision_tests = 0;
        let dw_step;
        let mut links_to_remove: Vec<Vec<usize>> = vec![Vec::new(); THREADS];
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
                        let mut energy_delta = 0.0;
                        if data_read.parts_to_remove.contains(&pid1) {
                            continue;
                        } else {
                            // ok
                        }
                        let p1 = data_read.parts[pid1];
                        let mut d_collision = Point { x: 0.0, y: 0.0 };
                        // TODO: store collisions for debugging
                        let mut _collisions = 0;
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
                                                    y: dpw.y * mass_factor * dot_vp / distance_sqrd,
                                                };
                                                d_collision -= acceleration;
                                                _collisions += 1;
                                                match (p1.kind, p2.kind) {
                                                    (Kind::Mouth, Kind::Mouth) => {}
                                                    (Kind::Mouth, _) => {
                                                        energy_delta += ENERGY_TRANSFER_EAT;
                                                    }
                                                    (_, Kind::Mouth) => {
                                                        energy_delta -= ENERGY_TRANSFER_EAT;
                                                    }
                                                    _ => {}
                                                }
                                            } else {
                                                // pass
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        tmp_energies_delta[pid1] = energy_delta;
                        tmp_speeds[pid1] = p1.p - p1.pp + d_collision;
                        tmp_directions[pid1].neighbour_count = 0.0;
                        tmp_directions[pid1].direction.x = 0.0;
                        tmp_directions[pid1].direction.y = 0.0;
                        match (p1.kind) {
                            Kind::Mouth => part_ok[pid1] = false,
                            _ => part_ok[pid1] = true,
                        }
                    }
                }
            }
            for (thid2, links) in data_read.links.iter().enumerate() {
                let dr = &drs_[thid2];
                for (link_id, link) in links.iter().enumerate() {
                    if data_read.parts_to_remove.contains(&link.pid1)
                        || dr.parts_to_remove.contains(&link.pid2)
                    {
                        links_to_remove[thid2].push(link_id);
                        continue;
                    } else {
                        // ok
                    }
                    let p1 = data_read.parts[link.pid1];
                    let p2 = dr.parts[link.pid2];
                    tmp_energies_delta[link.pid1] -=
                        (p1.energy - p2.energy) * ENERGY_TRANSFER;

                    match (p1.kind, p2.kind) {
                        (Kind::Mouth, Kind::Mouth) => {}
                        (Kind::Mouth, _) => part_ok[link.pid1] = true,
                        _ => {}
                    }

                    let distance_sqrd = distance_squared_wrap_around(&p1.p, &p2.p);
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
                        y: dpw.y * mass_factor * dot_vp / distance_sqrd,
                    };
                    let colliding = distance_sqrd < diameter * diameter;
                    let ndpw = normalize(dpw);
                    let link_force = ndpw * ((diameter * diameter - distance_sqrd) * LINK_STRENGTH);
                    tmp_directions[link.pid1].neighbour_count += 1.0;
                    tmp_directions[link.pid1].direction += &ndpw;
                    if colliding {
                        tmp_speeds[link.pid1] +=
                            &(link_force / p1.m * DELTA_TIME + acceleration * 0.5);
                    } else {
                        tmp_speeds[link.pid1] += &(link_force / p1.m * DELTA_TIME);
                    }
                }
            }
            for i in 0..WIDTH {
                for j in 0..HEIGHT {
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

                        match p1.kind {
                            Kind::Turbo => {
                                tmp_speeds[pid1].x -= tmp_directions[pid1].direction.x
                                    / tmp_directions[pid1].neighbour_count
                                    * 0.00001;
                                tmp_speeds[pid1].y -= tmp_directions[pid1].direction.y
                                    / tmp_directions[pid1].neighbour_count
                                    * 0.00001;
                            }
                            _ => {}
                        }

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
                        tmp_parts[tmp_count].energy = p1.energy + tmp_energies_delta[pid1];
                        tmp_parts[tmp_count].kind = p1.kind;
                        tmp_parts[tmp_count].m = p1.m;
                        tmp_pids[tmp_count] = pid_new;
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
            for j in (0..l).rev() {
                links_clone[i].remove(links_to_remove[i][j]);
            }
        }
        // {
        //     let mut dw = dws[thread_id].write().unwrap();
        //     let data_read = drs[thread_id].read().unwrap();
        //     for pid in data_read.parts_to_remove.iter() {
        //         dw.parts[*pid].kind = Kind::Invalid;
        //         dw.parts[*pid].energy = 0.0;
        //     }
        // }

        let mut entities_to_add: Vec<EntityToAdd> = Vec::new();

        {
            let mut dw = dws[thread_id].write().unwrap();
            dw.parts_to_remove.clear();
            // for old_pid in old_pids.iter() {
            //     dw.parts[*old_pid].kind = Kind::Invalid;
            //     dw.parts[*old_pid].energy = 0.0;
            // }
            for i in 0..tmp_count {
                let pid = tmp_pids[i];
                let pid_old = old_pids[i];
                let part = tmp_parts[i];
                dw.parts[pid].p.x = part.p.x;
                dw.parts[pid].p.y = part.p.y;
                dw.parts[pid].d = part.d;

                if (dw.parts[pid].d < DIAMETER_MIN) {
                    dw.parts[pid].d = (dw.parts[pid].d + GROWTH).min(DIAMETER_MIN);
                }

                assert!(part.d <= DIAMETER_MAX);
                assert!(part.d >= DIAMETER_NEW);
                dw.parts[pid].m = part.m;
                dw.parts[pid].kind = part.kind;
                dw.parts[pid].energy = if part.energy <= 0.0 || !part_ok[old_pids[i]] {
                    dw.parts_to_remove.insert(pid);
                    part.energy
                } else if part.energy > 1.0 {
                    match part.kind {
                        Kind::Core => {
                            entities_to_add.push(EntityToAdd {
                                source_thread_id: thread_id,
                                source_dna: 2,
                                total_energy: 0.9,
                                position: part.p
                                    + (tmp_directions[pid_old].direction
                                        / tmp_directions[pid_old].neighbour_count)
                                        * part.d * 2.0,
                            });
                            part.energy - 0.9
                        }
                        _ => part.energy,
                    }
                } else {
                    part.energy
                };

                dw.parts[pid].pp.x = part.pp.x;
                dw.parts[pid].pp.y = part.pp.y;
                dw.new_pids[pid_old] = pid;
            }
            dw.entities_to_add = entities_to_add;
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
                        for (thread_id_2, d) in dws_.iter().enumerate().skip(min) {
                            if d.step < dw_step && thread_id_2 != thread_id {
                                b = false;
                                break;
                            }
                            min = thread_id_2;
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
                            ds[thid1].links[thid2][i].pid1 =
                                ds[thid1].new_pids[ds[thid1].links[thid2][i].pid1];
                            ds[thid1].links[thid2][i].pid2 =
                                ds[thid2].new_pids[ds[thid1].links[thid2][i].pid2];
                        }
                    }
                }
            }
            // Add new entities
            {
                // TODO
                let plan = get_plan();
                for thid in 0..THREADS {
                    let entities_to_add = { dws[thid].read().unwrap().entities_to_add.clone() };
                    let mut dws_: Vec<RwLockWriteGuard<Data>> =
                        dws.iter().map(|x| x.write().unwrap()).collect();
                    for (i, entity) in entities_to_add.iter().enumerate() {
                        let rotation = 0.0;
                        let thread_id__ = dw_step % THREADS;
                        // println!("start add_entity");
                        add_entity_2(
                            &mut dws_,
                            &plan,
                            &entity.position,
                            rotation,
                            thread_id__,
                            entity.total_energy,
                        );
                        // println!("end   add_entity");
                    }
                }
            }
            // Update step to end all writes
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
                    for (i, d) in dws_.iter().enumerate().skip(min) {
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
        ends[dw_step % DATA_POINTS_COUNT] = SystemTime::now();
        if dw_step % 100 == 0 && thread_id == 0 {
            // TODO:
            // Why are they not the same ?
            let mut energy_total: Float = 0.0;
            let mut energy_total_2: Float = 0.0;
            let mut parts = 0;
            let mut depths: u32 = 0;
            {
                let datas: Vec<RwLockReadGuard<Data>> =
                    dws.iter().map(|x| x.read().unwrap()).collect();
                for thid in 0..THREADS {
                    // for part in datas[thid].parts.iter() {
                    //     energy_total += part.energy;
                    //     match part.kind  {
                    //         Kind::Invalid => {}
                    //         _ => {parts += 1}
                    //     }
                    // }
                    // for depth in datas[thid].depths.iter() {
                    //     depths += *depth as u32;
                    // }

                    for cid in 0..WIDTH_X_HEIGHT {
                        for k in 0..datas[thid].depths[cid] {
                            let pid = part_id(cid, k);
                            let part = datas[thid].parts[pid];
                            energy_total_2 += part.energy;
                        }
                    }
                }
            }

            let duration = ends[(dw_step + 1) % DATA_POINTS_COUNT].elapsed().unwrap()
                / DATA_POINTS_COUNT as u32;
            let duration_ = ends[(dw_step + 1) % DATA_POINTS_COUNT]
                .elapsed()
                .unwrap()
                .as_secs_f32()
                / DATA_POINTS_COUNT as Float;
            let cps = 1.0 / duration_;
            let mpps = (0.000001 / duration_) * (TOTAL_COUNT) as Float;
            let global_duration = start.elapsed().unwrap();
            let global_cps = dw_step as Float / start.elapsed().unwrap().as_secs_f32();
            let global_mpps = global_cps * (TOTAL_COUNT as Float) * 0.000001;
            println!(
                "\
Thread #{}
    current:
        utc:             {}
        step:            {}
        collision_tests: {}
        part count #1:   {}
        part count all:  {}
        energy:          {}
        energy 2:        {}
        depths:          {}
    last {}
        compute:         {:?}
        cps:             {}
        pps:             {:.2} MP/s
    average:
        cps:             {}
        pps:             {:.2} MP/s
    total:
        time:            {:?}",
                thread_id,
                Utc::now(),
                dw_step,
                collision_tests,
                tmp_count,
                parts,
                energy_total,
                energy_total_2,
                depths,
                DATA_POINTS_COUNT,
                duration,
                cps,
                mpps,
                global_cps,
                global_mpps,
                global_duration
            );
        }
        step = dw_step;
    }
}
