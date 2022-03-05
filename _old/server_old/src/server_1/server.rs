#![deny(warnings)]
use rand;
use rand::Rng;
use std::sync::RwLock;
pub const BLOCKS: usize = 4;
pub const CLIENT_BLOCKS: usize = 4;
const BASE_CAPACITY: usize = 10;
const TOTAL_COUNT: i32 = 100_000;
const THREADS: usize = 8;
const COUNT: i32 = TOTAL_COUNT / THREADS as i32;
const MODULO: usize = 100;
const TIMES_COUNT: usize = 100;
use crate::{
    part::Part,
    server_1::{
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
    pub pids: Vec<Vec<u128>>,
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
fn init(data: &Arc<RwLock<Data>>) {
    let mut d = data.write().unwrap();
    d.pids = vec![Vec::with_capacity(BASE_CAPACITY); BLOCKS*BLOCKS];
    let mut rng = rand::thread_rng();
    for _ in 0..COUNT {
        let a = 0.00001;
        add_part(&mut AddPartArgs {
            x: rng.gen::<f64>(),
            y: rng.gen::<f64>(),
            dx: rng.gen::<f64>() * a - a * 0.5,
            dy: rng.gen::<f64>() * a - a * 0.5,
            data: &mut d,
        });
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
    let mut dw_pids: Vec<Vec<u128>> =
        vec![Vec::with_capacity(BASE_CAPACITY); BLOCKS* BLOCKS];
    let mut dw_parts: HashMap<u128, Part> = HashMap::new();
    let dw_step;
    {
        let _drs_: Vec<RwLockReadGuard<Data>> = drs.iter().map(|x| x.read().unwrap()).collect();
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
        for pids in dr.pids.iter() {
            for pid in pids.iter() {
                let p1r = dr.parts.get(pid).unwrap();
                let dx = p1r.x - p1r.x_old;
                let dy = p1r.y - p1r.y_old;
                let x = (p1r.x + dx) % dr.width;
                let y = (p1r.y + dy) % dr.height;
                let i: usize = ((x * dr.width) % dr.width).floor() as usize;
                let j: usize = ((y * dr.height) % dr.height).floor() as usize;
                let ij = i + j * BLOCKS;
                dw_pids[ij].push(*pid);
                dw_parts.insert(
                    *pid,
                    Part {
                        x: x,
                        y: y,
                        x_old: x - dx,
                        y_old: y - dy,
                        colissions: 0,
                    },
                );
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
    pub dx: f64,
    pub dy: f64,
    pub data: &'a mut Data,
}
pub fn add_part(a: &mut AddPartArgs) {
    let i: usize = ((a.x * a.data.width) % a.data.width).floor() as usize;
    let j: usize = ((a.y * a.data.height) % a.data.height).floor() as usize;
    let ij = i + j * BLOCKS;
    let part_id: u128 = Uuid::new_v4().as_u128();
    a.data.parts.insert(
        part_id,
        Part {
            x: a.x,
            y: a.y,
            x_old: a.x - a.dx,
            y_old: a.y - a.dy,
            colissions: 0,
        },
    );
    a.data.pids[ij].push(part_id)
}
