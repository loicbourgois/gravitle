use crate::data::Data;
use crate::data::EntityToAdd;
use crate::entity::add_entity;
use crate::entity::add_part_energy;
use crate::maths::cross;
use crate::maths::delta_position_wrap_around;
use crate::maths::distance_squared_wrap_around;
use crate::maths::dot;
use crate::maths::normalize;
use crate::plan::random_dna;
use crate::plan::link_plan_to_ab_plan;
use crate::plan::dna_to_link_plan;
use crate::part::Part;
use crate::plan::get_plan_pyra;
use crate::plan::ab_plan_to_link_plan;
use crate::plan::get_plan_planner;
use crate::plan::get_plan_pusher;
use crate::plan::get_plan_aie;
use crate::plan::link_plan_to_dna;
use crate::plan::mutate_dna_inplace;
use crate::plan::Dna;
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
const MAX_DEPTH: Depth = 128;
const SIZE: usize = WIDTH * HEIGHT * MAX_DEPTH as usize;
pub const DIAMETER_NEW: Float = 0.05 / (DIMENSION as Float);
pub const DIAMETER_MIN: Float = 0.5 / (DIMENSION as Float);
pub const DIAMETER_MAX: Float = 0.6 / (DIMENSION as Float);
const GROWTH: Float = DIAMETER_MIN * 0.001;
const WIDTH_X_HEIGHT: usize = WIDTH * HEIGHT;
const WEBSOCKET_ASYNC: bool = false;
const DELTA_TIME: Float = 1.0 / 60.0;
const DATA_POINTS_COUNT: usize = 100;
const LINK_STRENGTH: Float = 5000.0;
const ENERGY_TRANSFER: Float = 0.2;
const ENERGY_TRANSFER_EAT: Float = 0.02;
const MEASURE_PARTS: bool = false;
const MEASURE_DEPTHS: bool = true;
const INIT_ENERGY_RATIO: Float = 0.5;
const TURBO_SPEED: Float = 0.00001;
const MAX_SPEED: Float = 0.0001;
const ACTIVITY_CHANGE_RATE: Float = 0.0;
pub const START_ACTIVITY: Float = 1.0;
const MUTATIONS: usize = 1;
const D_MOVE_RATIO: Float = 0.01;

pub fn cell_id(i: usize, j: usize) -> usize {
    i + j * WIDTH
}

fn home_dir() -> String {
    dirs::home_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()
}

pub fn part_id_next(cid: CellId, depths: &[Depth]) -> usize {
    let k = depths[cid];
    // assert!(k < MAX_DEPTH);
    // if k > MAX_DEPTH {
    //     println!("[WARN ] k > MAX_DEPTH: {} > {}", k, MAX_DEPTH);
    // }
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
            activity: 0.0,
            uuid: 0,
            r: 0,
            g: 0,
            b: 0,
        };
        SIZE
    ]
}

#[derive(Clone)]
pub struct DnaSave {
    pub parent_uuid: u128,
    pub dna: Dna,
}

#[derive(Clone)]
struct Wrapper {
    data1s: Vec<Arc<RwLock<Data>>>,
    data2s: Vec<Arc<RwLock<Data>>>,
    dnas: Arc<RwLock<HashMap<u128, DnaSave>>>,
    dna_save_file_path: String,
}
use std::fs::File;

pub async fn start() {
    let timer_init = SystemTime::now();
    println!("[ start ] Initializing");
    let start = Utc::now();
    let mut w = Wrapper {
        data1s: Vec::new(),
        data2s: Vec::new(),
        dnas: Arc::new(RwLock::new(HashMap::new())),
        dna_save_file_path: format!(
            "{}/github.com/loicbourgois/gravitle_local/dna/{}.csv",
            home_dir(),
            start
        ),
    };
    let _f = File::create(&w.dna_save_file_path).expect("Unable to create file");
    let mut rng = rand::thread_rng();
    for i in 0..THREADS {
        w.data1s.push(Arc::new(RwLock::new(Data {
            parts: init_parts(),
            depths: vec![0; WIDTH_X_HEIGHT],
            links: vec![Vec::new(); THREADS],
            step: 0,
            new_pids: vec![0; SIZE],
            parts_to_remove: HashSet::new(),
            entities_to_add: Vec::new(),
        })));
        w.data2s.push(Arc::new(RwLock::new(Data {
            parts: init_parts(),
            depths: vec![0; WIDTH_X_HEIGHT],
            links: vec![Vec::new(); THREADS],
            step: 0,
            new_pids: vec![0; SIZE],
            parts_to_remove: HashSet::new(),
            entities_to_add: Vec::new(),
        })));
        println!("Thread {}/{}", i + 1, THREADS);
    }

    {
        let mut datas: Vec<RwLockWriteGuard<Data>> =
            w.data1s.iter().map(|x| x.write().unwrap()).collect();
        let mut p_count = 0;
        loop {
            let plan = match rng.gen_range(0..5) {
                0 => get_plan_pusher(),
                1 => get_plan_planner(),
                2 => get_plan_pyra(),
                3 => get_plan_aie(),
                _ => link_plan_to_ab_plan(&dna_to_link_plan(&random_dna())),
            };
            let dna_save = DnaSave {
                dna: link_plan_to_dna(&ab_plan_to_link_plan(&plan)),
                parent_uuid: 0,
            };
            let part_plans_count = {
                let mut c = 2;
                for part in plan.part_plans.iter() {
                    match part.k {
                        Kind::Invalid => {
                            break;
                        }
                        _ => {
                            c += 1;
                        }
                    }
                }
                c
            };
            add_entity(
                &mut datas,
                &Point {
                    x: rng.gen::<Float>(),
                    y: rng.gen::<Float>(),
                },
                rng.gen::<Float>(),
                rng.gen_range(0..THREADS),
                part_plans_count as Float * INIT_ENERGY_RATIO,
                &mut w.dnas.write().unwrap(),
                &dna_save,
                &w.dna_save_file_path,
            );
            p_count += part_plans_count;
            println!("Parts:  {:1}/{}", (p_count as Float) , TOTAL_COUNT);
            if (p_count as Float) >= TOTAL_COUNT as Float {
                break
            }
        }
    }
    let mut handles = Vec::new();
    for i in 0..THREADS {
        let w_ = w.clone();
        handles.push(thread::spawn(move || {
            compute_loop(&w_, i);
        }));
    }
    let senders: websocket::Senders = Arc::new(Mutex::new(HashMap::new()));
    if WEBSOCKET_ASYNC {
        websocket_async::serve_async(&senders).await;
    } else {
        websocket::serve(&senders);
        websocket::send(&websocket::SendArgs {
            senders: &senders,
            datas: &w.data1s,
        });
    }
    println!(
        "[  end  ] Initializing: {:?}",
        timer_init.elapsed().unwrap()
    );
    for handle in handles {
        handle.join().unwrap();
    }
}

#[derive(Clone)]
struct Direction {
    neighbour_count: Float,
    direction: Point,
}

fn compute_loop(w: &Wrapper, thread_id: usize) {
    let d1s = &w.data1s;
    let d2s = &w.data2s;
    let mut rng = rand::thread_rng();
    let mut tmp_parts = init_parts();
    let mut tmp_speeds: Vec<Point> = vec![Point { x: 0.0, y: 0.0 }; SIZE];
    let mut tmp_moves: Vec<Point> = vec![Point { x: 0.0, y: 0.0 }; SIZE];
    let mut tmp_pids: Vec<usize> = vec![0; SIZE];
    let mut tmp_energies_delta: Vec<Float> = vec![0.0; SIZE];
    let mut part_ok: Vec<u8> = vec![100; SIZE];
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
            //
            // Collisions
            //
            for a in 0..WIDTH_X_HEIGHT {
                let i = a % DIMENSION;
                let j = a / DIMENSION;
                let cid1 = cell_id(i, j);
                let i2s = [(i + WIDTH_LESS) % WIDTH, i, (i + WIDTH_MORE) % WIDTH];
                let j2s = [(j + HEIGHT_LESS) % HEIGHT, j, (j + HEIGHT_MORE) % HEIGHT];
                let cid2s = [
                    cell_id(i2s[0], j2s[0]),
                    cell_id(i2s[0], j2s[1]),
                    cell_id(i2s[0], j2s[2]),
                    cell_id(i2s[1], j2s[0]),
                    cell_id(i2s[1], j2s[1]),
                    cell_id(i2s[1], j2s[2]),
                    cell_id(i2s[2], j2s[0]),
                    cell_id(i2s[2], j2s[1]),
                    cell_id(i2s[2], j2s[2]),
                ];
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
                    let mut d_move_collision = Point { x: 0.0, y: 0.0 };
                    // TODO: store collisions for debugging
                    let mut _collisions = 0;
                    for cid2 in cid2s {
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
                                    let distance_sqrd = distance_squared_wrap_around(&p1.p, &p2.p);
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
                                        let dpwn = normalize(dpw);
                                        d_move_collision += &(dpwn * diameter * D_MOVE_RATIO);
                                        _collisions += 1;
                                        match (p1.kind, p2.kind) {
                                            (Kind::Mouth, Kind::Mouth) => {
                                                // feeding baby
                                                if p1.energy > p2.energy {
                                                    energy_delta -= ENERGY_TRANSFER_EAT;
                                                } else if p1.energy < p2.energy {
                                                    energy_delta += ENERGY_TRANSFER_EAT;
                                                } else {
                                                }
                                                // energy_delta -= ENERGY_TRANSFER_EAT;
                                            }
                                            (Kind::Mouth, _) => {
                                                energy_delta += ENERGY_TRANSFER_EAT;
                                            }
                                            (_, Kind::Mouth) => {
                                                energy_delta -= ENERGY_TRANSFER_EAT;
                                            }
                                            _ => {
                                                if distance_sqrd < diameter * diameter * 0.5 * 0.5 {
                                                    if p1.energy > p2.energy {
                                                        energy_delta += ENERGY_TRANSFER_EAT;
                                                    } else if p1.energy < p2.energy {
                                                        energy_delta -= ENERGY_TRANSFER_EAT;
                                                    } else {
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        // pass
                                    }
                                }
                            }
                        }
                    }
                    tmp_energies_delta[pid1] = energy_delta;
                    tmp_speeds[pid1] = p1.p - p1.pp + d_collision;
                    tmp_moves[pid1] = d_move_collision;
                    tmp_directions[pid1].neighbour_count = 0.0;
                    tmp_directions[pid1].direction.x = 0.0;
                    tmp_directions[pid1].direction.y = 0.0;
                    match p1.kind {
                        Kind::Mouth => part_ok[pid1] = 0,
                        _ => part_ok[pid1] = 100,
                    }
                }
            }
            //
            // Links
            //
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
                    tmp_energies_delta[link.pid1] -= (p1.energy - p2.energy) * ENERGY_TRANSFER;
                    match (p1.kind, p2.kind) {
                        (Kind::Mouth, Kind::Mouth) => {}
                        (Kind::Mouth, _) => part_ok[link.pid1] += 1,
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
            //
            // Update
            //
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
                        let direction =
                            tmp_directions[pid1].direction / tmp_directions[pid1].neighbour_count;
                        if let Kind::Turbo = p1.kind {
                            tmp_speeds[pid1].x -= direction.x * TURBO_SPEED * p1.activity;
                            tmp_speeds[pid1].y -= direction.y * TURBO_SPEED * p1.activity;
                        }
                        // tmp_speeds[pid1].x = tmp_speeds[pid1].x;//.max(-MAX_SPEED).min(MAX_SPEED);
                        // tmp_speeds[pid1].y = tmp_speeds[pid1].y;//.max(-MAX_SPEED).min(MAX_SPEED);

                        let mut dx = tmp_speeds[pid1].x;
                        let mut dy = tmp_speeds[pid1].y;

                        if dx > MAX_SPEED {
                            dx = MAX_SPEED * 0.9;
                            //println!("speed issue dx > MAX_SPEED");
                        }
                        if dx < -MAX_SPEED {
                            dx = -MAX_SPEED * 0.9;
                            //println!("speed issue dx < -MAX_SPEED");
                        }
                        if dy > MAX_SPEED {
                            dy = MAX_SPEED * 0.9;
                            //println!("speed issue dy > MAX_SPEED");
                        }
                        if dy < -MAX_SPEED {
                            dy = -MAX_SPEED * 0.9;
                            //println!("speed issue dy < -MAX_SPEED");
                        }

                        if let Kind::Grip = p1.kind {
                            let direction_perpendicular = Point {
                                x: direction.y,
                                y: -direction.x,
                            };
                            // let pa1 = pA - direction_perpendicular;
                            // let pa2 = pA + direction_perpendicular
                            let cross_ = cross(&direction_perpendicular, &tmp_speeds[pid1]);
                            if cross_ > 0.0 {
                                dx *= 0.5;
                                dy *= 0.5;
                            }
                        }

                        let x = (p1.p.x + dx + 1.0 + tmp_moves[pid1].x).fract();
                        let y = (p1.p.y + dy + 1.0 + tmp_moves[pid1].y).fract();
                        let x_ = x - dx;
                        let y_ = y - dy;
                        let i_new: usize = (x * WIDTH as Float) as usize;
                        let j_new: usize = (y * HEIGHT as Float) as usize;
                        let cid_new: CellId = cell_id(i_new, j_new);
                        let pid_new = part_id_next(cid_new, &tmp_depths);
                        if pid_new < SIZE {
                            // /!\ COPY VALUES 1/2
                            tmp_depths[cid_new] += 1;
                            tmp_parts[tmp_count].p.x = x;
                            tmp_parts[tmp_count].p.y = y;
                            tmp_parts[tmp_count].pp.x = x_;
                            tmp_parts[tmp_count].pp.y = y_;
                            tmp_parts[tmp_count].d = p1.d;
                            tmp_parts[tmp_count].activity = p1.activity;
                            tmp_parts[tmp_count].energy = p1.energy + tmp_energies_delta[pid1];
                            tmp_parts[tmp_count].kind = p1.kind;
                            tmp_parts[tmp_count].m = p1.m;
                            tmp_parts[tmp_count].r = p1.r;
                            tmp_parts[tmp_count].g = p1.g;
                            tmp_parts[tmp_count].b = p1.b;
                            tmp_parts[tmp_count].uuid = p1.uuid;
                            tmp_pids[tmp_count] = pid_new;
                            old_pids[tmp_count] = pid1;
                            tmp_count += 1;
                        } else {
                            println!("[WARN ] pid_new >= SIZE: {} >= {}", pid_new, SIZE);
                        }
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
        let mut entities_to_add: Vec<EntityToAdd> = Vec::new();
        {
            let mut dw = dws[thread_id].write().unwrap();
            dw.parts_to_remove.clear();
            for i in 0..tmp_count {
                // /!\ COPY VALUES 2/2
                let pid = tmp_pids[i];
                let pid_old = old_pids[i];
                let part = tmp_parts[i];
                dw.parts[pid].p.x = part.p.x;
                dw.parts[pid].p.y = part.p.y;
                dw.parts[pid].uuid = part.uuid;
                dw.parts[pid].r = part.r;
                dw.parts[pid].g = part.g;
                dw.parts[pid].b = part.b;
                dw.parts[pid].d = part.d;
                if dw.parts[pid].d < DIAMETER_MIN {
                    dw.parts[pid].d = (dw.parts[pid].d + GROWTH).min(DIAMETER_MIN);
                } else {
                    match part.kind {
                        Kind::Muscle => {
                            dw.parts[pid].d = DIAMETER_MIN
                                + (DIAMETER_MAX - DIAMETER_MIN)
                                    * (1.0 + ((step as Float) * 0.1).sin())
                                    * 0.5;
                        }
                        _ => {
                            dw.parts[pid].d = (dw.parts[pid].d + GROWTH).min(DIAMETER_MAX);
                        }
                    }
                }
                dw.parts[pid].m = part.m;
                dw.parts[pid].kind = part.kind;
                dw.parts[pid].activity = (part.activity
                    + rng.gen::<Float>() * ACTIVITY_CHANGE_RATE
                    - ACTIVITY_CHANGE_RATE * 0.5)
                    .max(0.0)
                    .min(1.0);
                dw.parts[pid].energy =
                    if part.energy <= 0.0 || part_ok[old_pids[i]] < 3 || part.energy > 2.0 {
                        dw.parts_to_remove.insert(pid);
                        part.energy
                    } else if part.energy > 1.0 {
                        match part.kind {
                            Kind::Core => {
                                entities_to_add.push(EntityToAdd {
                                    source_thread_id: thread_id,
                                    source_dna_id: part.uuid,
                                    total_energy: 0.9,
                                    position: part.p
                                        + (tmp_directions[pid_old].direction
                                            / tmp_directions[pid_old].neighbour_count)
                                            * part.d
                                            * 2.0,
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

            // remove dnas
            {
                let drs_: Vec<RwLockReadGuard<Data>> =
                    drs.iter().map(|x| x.read().unwrap()).collect();
                let mut dnas = w.dnas.write().unwrap();
                for dr in drs_.iter().take(THREADS) {
                    for part_to_remove in &dr.parts_to_remove {
                        let p = dr.parts[*part_to_remove];
                        // TODO: save dna to file
                        dnas.remove(&p.uuid);
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
                for thid in 0..THREADS {
                    let entities_to_add = { dws[thid].read().unwrap().entities_to_add.clone() };
                    let mut dws_: Vec<RwLockWriteGuard<Data>> =
                        dws.iter().map(|x| x.write().unwrap()).collect();
                    for (_, entity) in entities_to_add.iter().enumerate() {
                        // println!("#{}: adding {}/{}", thid, i+1, entities_to_add.len());
                        let mut dnas = w.dnas.write().unwrap();
                        match dnas.get(&entity.source_dna_id) {
                            Some(dna_save) => {
                                let mut dna = dna_save.dna;
                                for _ in 0..MUTATIONS {
                                    mutate_dna_inplace(&mut dna);
                                }
                                let dna_save = DnaSave {
                                    dna,
                                    parent_uuid: entity.source_dna_id,
                                };
                                let rotation = 0.0;
                                let thread_id__ = dw_step % THREADS;
                                add_entity(
                                    &mut dws_,
                                    &entity.position,
                                    rotation,
                                    thread_id__,
                                    entity.total_energy,
                                    &mut dnas,
                                    &dna_save,
                                    &w.dna_save_file_path,
                                );
                            }
                            None => {
                                println!(
                                    "[ERROR] none dna_save for source_dna_id: {}",
                                    entity.source_dna_id
                                );
                            }
                        }
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
            // TODO: Why are they not the same ?
            let mut energy_total: Float = 0.0;
            let mut energy_total_2: Float = 0.0;
            let mut parts = 0;
            let mut depths: u32 = 0;
            {
                let datas: Vec<RwLockReadGuard<Data>> =
                    dws.iter().map(|x| x.read().unwrap()).collect();
                for data in datas.iter().take(THREADS) {
                    if MEASURE_PARTS {
                        for part in data.parts.iter() {
                            energy_total += part.energy;
                            match part.kind {
                                Kind::Invalid => {}
                                _ => parts += 1,
                            }
                        }
                    }
                    if MEASURE_DEPTHS {
                        for depth in data.depths.iter() {
                            depths += *depth as u32;
                        }
                    }
                    for cid in 0..WIDTH_X_HEIGHT {
                        for k in 0..data.depths[cid] {
                            let pid = part_id(cid, k);
                            let part = data.parts[pid];
                            energy_total_2 += part.energy;
                        }
                    }
                }
            }
            {
                let energy_max = TOTAL_COUNT as Float * INIT_ENERGY_RATIO;
                let mut energy_total_tmp = energy_total_2;
                loop {
                    let position = Point {
                        x: rng.gen::<Float>(),
                        y: rng.gen::<Float>(),
                    };
                    let thread_id = rng.gen_range(0..THREADS);
                    let data = &mut dws[thread_id].write().unwrap();
                    add_part_energy(data, &position);
                    energy_total_tmp += 1.0;
                    if energy_total_tmp > energy_max {
                        break;
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
            let mpps = (0.000001 / duration_) * (depths) as Float;
            let global_duration = start.elapsed().unwrap();
            let global_cps = dw_step as Float / start.elapsed().unwrap().as_secs_f32();
            let global_mpps = global_cps * (depths as Float) * 0.000001;
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
        dnas:            {}
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
                w.dnas.read().unwrap().len(),
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
