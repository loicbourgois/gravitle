#![deny(warnings)]
use rand;
use rand::Rng;
use std::sync::RwLock;
pub const BLOCKS: usize = 16 * 8 * 2;
pub const CLIENT_BLOCKS: usize = BLOCKS / 8;
const BASE_CAPACITY: usize = 10;
const TOTAL_COUNT: i32 = 40_000;
const THREADS: usize = 4;
const COUNT: i32 = TOTAL_COUNT / THREADS as i32;
const MODULO: usize = 100;
const TIMES_COUNT: usize = 100;
const DIAMETER: f64 = 0.001;
use crate::{
    part::Part,
    maths::{
        distance_squared_wrap_around,
        delta_position_wrap_around,
        dot,
        normalize
    },
    server_2::{
        websocket::{send, serve, SendArgs, Senders, ServeArgs}
    },
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, RwLockReadGuard},
    thread,
    time::Duration,
    time::SystemTime,
};
use uuid::Uuid;
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
enum FirstMessageRequest {
    CreateSender,
    CreateReceiver,
}
#[derive(Serialize, Deserialize, Debug)]
struct FirstMessage {
    request: FirstMessageRequest,
    uuid: Uuid,
}
// type Senders = Arc<Mutex<HashMap<u128, WebSocket<TcpStream>>>>;
// type Receivers = Arc<Mutex<HashMap<u128, WebSocket<TcpStream>>>>;
#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub step: usize,
    pub pids: Vec<Vec<Vec<u128>>>,
    pub parts: HashMap<u128, Part>,
    pub width: f64,
    pub height: f64,
    pub id: i32,
}
// #[derive(Serialize, Deserialize, Debug)]
// struct DataClient {
//     step: usize,
//     pids: Vec<Vec<HashSet<u128>>>,
//     parts: HashMap<u128, Part>,
//     width: f64,
//     height: f64,
// }
pub fn main() {
    // let server = TcpListener::bind("127.0.0.1:8000").unwrap();
    // let senders: Senders = Arc::new(Mutex::new(HashMap::new()));
    // let receivers: Receivers = Arc::new(Mutex::new(HashMap::new()));
    let mut data1s: Vec<Arc<RwLock<Data>>> = Vec::new();
    let mut data2s: Vec<Arc<RwLock<Data>>> = Vec::new();
    //let mut rng = rand::thread_rng();
    for i in 0..THREADS {
        data1s.push(Arc::new(RwLock::new(Data {
            step: 0,
            parts: HashMap::new(),
            pids: Vec::new(),
            width: BLOCKS as f64,
            height: BLOCKS as f64,
            id: i as i32,
        })));
        data2s.push(Arc::new(RwLock::new(Data {
            step: 0,
            parts: HashMap::new(),
            pids: Vec::new(),
            width: BLOCKS as f64,
            height: BLOCKS as f64,
            id: i as i32,
        })));
        init(&data1s[i]);
        let dr = data1s[i].read().unwrap();
        let mut dw = data2s[i].write().unwrap();
        dw.step = dr.step;
        dw.parts = dr.parts.clone();
        dw.pids = dr.pids.clone();
        dw.width = dr.width;
        dw.height = dr.height;
        dw.id = dr.id;
    }
    for i in 0..THREADS {
        let d1s = data1s.clone();
        let d2s = data2s.clone();
        thread::spawn(move || {
            compute_loop(&d1s, &d2s, i);
        });
    }
    let senders: Senders = Arc::new(Mutex::new(HashMap::new()));
    serve(&ServeArgs { senders: &senders });
    send(&SendArgs {
        datas: &data1s,
        senders: &senders,
    });
    loop {
        thread::sleep(Duration::from_millis(1000));
    }
}
fn compute_loop(d1s: &Vec<Arc<RwLock<Data>>>, d2s: &Vec<Arc<RwLock<Data>>>, i: usize) {
    let mut times: Vec<SystemTime> = Vec::new();
    let mut step = 0;
    let mut data = ComputeArgs {
        times: &mut times,
        step: &mut step,
        d1s: &d1s,
        d2s: &d2s,
        i: i,
    };
    loop {
        compute(&mut data);
        thread::sleep(Duration::from_millis(1));
    }
}
pub struct ComputeArgs<'a> {
    pub times: &'a mut Vec<SystemTime>,
    pub d1s: &'a Vec<Arc<RwLock<Data>>>,
    pub d2s: &'a Vec<Arc<RwLock<Data>>>,
    pub step: &'a mut usize,
    pub i: usize,
}
fn compute(arg: &mut ComputeArgs) {
    let (drs, dws) = {
        if *arg.step % 2 == 0 {
            (arg.d1s, arg.d2s)
        } else {
            (arg.d2s, arg.d1s)
        }
    };
    {
        loop {
            let mut b = true;
            {
                let drs_: Vec<RwLockReadGuard<Data>> =
                    drs.iter().map(|x| x.read().unwrap()).collect();
                let dr = drs[arg.i].read().unwrap();
                for d in drs_.iter() {
                    if d.step < dr.step {
                        b = false;
                    }
                }
            }
            if b {
                break;
            }
            thread::sleep(Duration::from_millis(1));
        }
    }
    let mut dw_pids: Vec<Vec<Vec<u128>>> =
        vec![vec![Vec::with_capacity(BASE_CAPACITY); BLOCKS];BLOCKS];
    let mut dw_parts: HashMap<u128, Part> = HashMap::new();
    let dw_step;
    {
        let drs_: Vec<RwLockReadGuard<Data>> = drs.iter().map(|x| x.read().unwrap()).collect();
        let dr = drs[arg.i].read().unwrap();
        dw_step = dr.step + 1;
        *arg.step = dw_step;
        arg.times.push(SystemTime::now());
        if (dw_step % MODULO) == 1 {
            println!("step: {}", dr.step);
            if arg.times.len() > TIMES_COUNT {
                arg.times.drain(0..1);
                let compute = arg.times[TIMES_COUNT - 1]
                    .duration_since(arg.times[0])
                    .unwrap()
                    / TIMES_COUNT.try_into().unwrap();
                println!("id:       {:?}", dr.id);
                println!("cps:      {:?}", 1.0 / compute.as_secs_f32());
                println!("compute:  {:?}", compute);
                println!(
                    "pps:      {:.2} MP/s",
                    (0.000001 / compute.as_secs_f32()) * (COUNT * THREADS as i32) as f32
                );
            }
        }
        for pids_ in dr.pids.iter() {
            for pids in pids_.iter() {
                for pid in pids.iter() {
                    let p1 = dr.parts.get(pid).unwrap();
                    let i_: usize = ((p1.x * dr.width) % dr.width).floor() as usize;
                    let j_: usize = ((p1.y * dr.height) % dr.height).floor() as usize;
                    let mut colissions = 0;
                    let aa = 1;
                    let i_min = (i_+BLOCKS-aa)%BLOCKS;
                    let i_max = (i_+BLOCKS+aa)%BLOCKS;
                    let j_min = (j_+BLOCKS-aa)%BLOCKS;
                    let j_max = (j_+BLOCKS+aa)%BLOCKS;
                    let v1x = p1.x - p1.x_old;
                    let v1y = p1.y - p1.y_old;
                    let v1z = p1.z - p1.z_old;
                    let mut fx = 0.0; // Forces
                    let mut fy = 0.0;
                    let mut fz = 0.0;
                    let mut dx_collision = 0.0; // collision
                    let mut dy_collision = 0.0;
                    let mut dz_collision = 0.0;
                    for dr2 in drs_.iter() {
                        for i2 in i_min..=i_max {
                            for j2 in j_min..=j_max {
                                for pid2 in dr2.pids[i2][j2].iter() {
                                    if pid != pid2 {
                                        let p2 = dr2.parts.get(pid2).unwrap();
                                        let d_square = distance_squared_wrap_around(
                                            p1.x,
                                            p1.y,
                                            p1.z,
                                            p2.x,
                                            p2.y,
                                            p2.z,
                                        );
                                        let dpw = delta_position_wrap_around(p1.x, p1.y, 0.0, p2.x, p2.y, 0.0);
                                        if d_square < DIAMETER*DIAMETER*2.0 {
                                            let norm = normalize(dpw);
                                            let strength = 10.0;
                                            fx += norm.0 * (DIAMETER*DIAMETER - d_square)*strength;
                                            fy += norm.1 * (DIAMETER*DIAMETER - d_square)*strength;
                                            fz += norm.2 * (DIAMETER*DIAMETER - d_square)*strength;
                                        }
                                        if d_square < DIAMETER*DIAMETER {
                                            // https://en.wikipedia.org/wiki/Elastic_collision#Two-dimensional_collision_with_two_moving_objects
                                            colissions += 1;
                                            let v2x = p2.x - p2.x_old;
                                            let v2y = p2.y - p2.y_old;
                                            let v2z = p2.z - p2.z_old;
                                            let dvx = v1x - v2x;
                                            let dvy = v1y - v2y;
                                            let dvz = v1z - v2z;
                                            // TODO
                                            // let mass_factor = 2.0 * p1.mass / (p1.mass + p2.mass)
                                            let mass_factor = 1.0;
                                            let dot_vp = dot(dvx, dvy, dvz, dpw.0, dpw.1, dpw.2);
                                            let acc_x = dpw.0 * mass_factor * dot_vp / d_square;
                                            let acc_y = dpw.1 * mass_factor * dot_vp / d_square;
                                            let acc_z = dpw.2 * mass_factor * dot_vp / d_square;
                                            dx_collision -= acc_x * 0.5;
                                            dy_collision -= acc_y * 0.5;
                                            dz_collision -= acc_z * 0.5;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    let p1_mass = 1.0;
                    let delta_time = 1.0/60.0;
                    let acc_x = fx / p1_mass;
                    let acc_y = fy / p1_mass;
                    let acc_z = fz / p1_mass;
                    let max_speed = 1.0;
                    let speed_x = (p1.x - p1.x_old + acc_x * delta_time + dx_collision).max(-max_speed).min(max_speed);
                    let speed_y = (p1.y - p1.y_old + acc_y * delta_time + dy_collision).max(-max_speed).min(max_speed);
                    let speed_z = (p1.z - p1.z_old + acc_z * delta_time + dz_collision).max(-max_speed).min(max_speed);

                    let x = (p1.x + speed_x).fract();
                    let y = (p1.y + speed_y).fract();
                    let z = (p1.z + speed_z).fract() * 0.0;
                    let x_old = x - speed_x;
                    let y_old = y - speed_y;
                    let z_old = z - speed_z * 0.0;

                    let i: usize = ((x * dr.width) % dr.width).floor() as usize;
                    let j: usize = ((y * dr.height) % dr.height).floor() as usize;
                    dw_pids[i][j].push(*pid);
                    dw_parts.insert(
                        *pid,
                        Part {
                            x: x,
                            y: y,
                            z: z,
                            x_old: x_old,
                            y_old: y_old,
                            z_old: z_old,
                            colissions: colissions
                        },
                    );
                }
            }
        }
    }
    {
        let mut dw = dws[arg.i].write().unwrap();
        dw.pids = dw_pids;
        dw.parts = dw_parts;
        dw.step = dw_step;
    }
}
pub struct AddPartArgs<'a> {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub dx: f64,
    pub dy: f64,
    pub dz: f64,
    pub data: &'a mut Data,
}
fn init(data: &Arc<RwLock<Data>>) {
    let mut d = data.write().unwrap();
    d.pids = vec![vec![Vec::with_capacity(BASE_CAPACITY); BLOCKS];BLOCKS];
    let mut rng = rand::thread_rng();
    for _ in 0..COUNT {
        let a = 0.00001;
        add_part(&mut AddPartArgs {
            x: rng.gen::<f64>(),
            y: rng.gen::<f64>(),
            z: 0.0,
            dx: rng.gen::<f64>() * a - a * 0.5,
            dy: rng.gen::<f64>() * a - a * 0.5,
            dz: 0.0,
            data: &mut d,
        });
    }
}
pub fn add_part(a: &mut AddPartArgs) {
    let i: usize = ((a.x * a.data.width) % a.data.width).floor() as usize;
    let j: usize = ((a.y * a.data.height) % a.data.height).floor() as usize;
    let part_id: u128 = Uuid::new_v4().as_u128();
    a.data.parts.insert(
        part_id,
        Part {
            x: a.x,
            y: a.y,
            z: a.z,
            x_old: a.x - a.dx,
            y_old: a.y - a.dy,
            z_old: a.z - a.dz,
            colissions: 0,
        },
    );
    a.data.pids[i][j].push(part_id)
}
