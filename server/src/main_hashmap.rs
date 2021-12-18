const TIMES_COUNT: usize = 100;
const SIZE: usize = 100;
// const BASE_CAPACITY: usize = 10;
const COUNT: i32 = 100_000;
use rand;
use rand::Rng;
use std::net::TcpListener;
use std::sync::RwLock;
use tungstenite::accept;
use crate::part::Part;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
    thread,
    time::{Duration, SystemTime},
};
use tungstenite::{Message, WebSocket};
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
use std::net::TcpStream;
type Senders = Arc<Mutex<HashMap<u128, WebSocket<TcpStream>>>>;
type Receivers = Arc<Mutex<HashMap<u128, WebSocket<TcpStream>>>>;
#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub step: usize,
    pub pids: HashMap<(u32, u32), HashSet<u128>>,
    pub parts: HashMap<u128, Part>,
    pub width: f64,
    pub height: f64,
}
#[derive(Serialize, Deserialize, Debug)]
struct DataClient {
    step: usize,
    pids: Vec<Vec<HashSet<u128>>>,
    parts: HashMap<u128, Part>,
    width: f64,
    height: f64,
}
pub fn main() {
    let server = TcpListener::bind("127.0.0.1:8000").unwrap();
    let senders: Senders = Arc::new(Mutex::new(HashMap::new()));
    let receivers: Receivers = Arc::new(Mutex::new(HashMap::new()));
    let data = Arc::new(RwLock::new(Data {
        step: 0,
        parts: HashMap::new(),
        pids: HashMap::new(),
        width: SIZE as f64,
        height: SIZE as f64,
    }));
    {
        let data_ = data.clone();
        let senders_ = senders.clone();
        thread::spawn(move || {
            loop {
                let d = {
                    let d = data_.read().unwrap();
                    Data {
                        step: d.step,
                        parts: d.parts.clone(),
                        pids: d.pids.clone(),
                        height: d.height,
                        width: d.width,
                    }
                };
                let mut data_client = DataClient {
                    step: d.step,
                    // parts: d.parts.clone(),
                    parts: HashMap::new(),
                    pids: vec![vec![HashSet::new(); SIZE]; SIZE],
                    height: d.height,
                    width: d.width,
                };
                for ((i, j), pids) in d.pids.iter() {
                    if ((*i as i32) - 250).abs() < 30 && ((*j as i32) - 250).abs() < 30 {
                        //data_client.pids[*i as usize][*j as usize] = pids.clone();
                        for pid in pids {
                            let part = d.parts.get(pid).unwrap();
                            data_client.parts.insert(*pid, *part);
                        }
                    }
                }
                let mut sender_to_delete: HashSet<u128> = HashSet::new();
                for (_k, sender) in senders_.lock().unwrap().iter_mut() {
                    match serde_json::to_string(&data_client) {
                        Ok(d_string) => {
                            match sender.write_message(Message::Text(d_string)) {
                                Ok(_) => {}
                                Err(e) => {
                                    println!("error A: {} {:?}", *_k, e);
                                    sender_to_delete.insert(*_k);
                                }
                            };
                        }
                        Err(e) => {
                            println!("error B: {:?}", e)
                        }
                    }
                }
                for k in sender_to_delete.iter() {
                    senders_.lock().unwrap().remove(k);
                }
                thread::sleep(Duration::from_millis(10));
            }
        });
    }
    thread::spawn(move || {
        for stream in server.incoming() {
            println!("new stream");
            let senders_ = senders.clone();
            let receivers_ = receivers.clone();
            thread::spawn(move || {
                let mut websocket = accept(stream.unwrap()).unwrap();
                let message = websocket.read_message().unwrap().to_string();
                match serde_json::from_str::<FirstMessage>(&message) {
                    first_message => match first_message {
                        Ok(m) => match m.request {
                            FirstMessageRequest::CreateSender => {
                                senders_.lock().unwrap().insert(m.uuid.as_u128(), websocket);
                            }
                            FirstMessageRequest::CreateReceiver => {
                                receivers_
                                    .lock()
                                    .unwrap()
                                    .insert(m.uuid.as_u128(), websocket);
                            }
                        },
                        Err(e) => {
                            println!("{:?}", e)
                        }
                    },
                }
            });
        }
    });
    init(&data);
    compute_loop(&data);
}
fn init(data: &Arc<RwLock<Data>>) {
    let mut d = data.write().unwrap();
    let mut rng = rand::thread_rng();
    for _ in 0..COUNT {
        let a = 0.1;
        add_part(&mut AddPartArgs {
            x: d.width * rng.gen::<f64>(),
            y: d.height * rng.gen::<f64>(),
            dx: rng.gen::<f64>() * a - a * 0.5,
            dy: rng.gen::<f64>() * a - a * 0.5,
            data: &mut d,
        });
    }
}
fn compute_loop(data: &Arc<RwLock<Data>>) {
    let mut times = Vec::new();
    let mut dw = {
        let d = data.read().unwrap();
        Data {
            step: d.step,
            parts: d.parts.clone(),
            pids: d.pids.clone(),
            height: d.height,
            width: d.width,
        }
    };
    let mut c = ComputeArgs {
        times: &mut times,
        dw: &mut dw,
        data: &data,
    };
    loop {
        compute(&mut c);
    }
}
pub struct ComputeArgs<'a> {
    pub dw: &'a mut Data,
    pub times: &'a mut Vec<SystemTime>,
    pub data: &'a Arc<RwLock<Data>>,
}
fn compute(x: &mut ComputeArgs) {
    let dr = {
        let d = x.data.read().unwrap();
        Data {
            step: d.step,
            parts: d.parts.clone(),
            pids: d.pids.clone(),
            height: d.height,
            width: d.width,
        }
    };
    let mut dw_pids: HashMap<(u32, u32), HashSet<u128>> = HashMap::new();
    x.dw.step = dr.step + 1;
    x.times.push(SystemTime::now());
    if (x.dw.step % 10) == 0 {
        println!("step: {}", dr.step);
        if x.times.len() > TIMES_COUNT {
            x.times.drain(0..1);
            let compute = x.times[TIMES_COUNT - 1].duration_since(x.times[0]).unwrap()
                / TIMES_COUNT.try_into().unwrap();
            println!("cps:      {:?}", 1.0 / compute.as_secs_f32());
            println!("compute:  {:?}", compute);
        }
    }
    for (_, pids) in dr.pids.iter() {
        for pid in pids.iter() {
            let p1r = dr.parts.get(pid).unwrap();
            let dx = p1r.x - p1r.x_old;
            let dy = p1r.y - p1r.y_old;
            let x_ = (p1r.x + dx) % dr.width;
            let y = (p1r.y + dy) % dr.height;
            let i: u32 = (x_ % dr.width).floor() as u32;
            let j: u32 = (y % dr.height).floor() as u32;
            match dw_pids.get_mut(&(i, j)) {
                Some(hashset) => {
                    hashset.insert(*pid);
                }
                None => {
                    let mut hashset = HashSet::new();
                    hashset.insert(*pid);
                    dw_pids.insert((i, j), hashset);
                }
            }
            match x.dw.parts.get_mut(pid) {
                Some(p) => {
                    *p = Part {
                        x: x_,
                        y: y,
                        x_old: x_ - dx,
                        y_old: y - dy,
                    };
                }
                None => {
                    x.dw.parts.insert(
                        *pid,
                        Part {
                            x: x_,
                            y: y,
                            x_old: x_ - dx,
                            y_old: y - dy,
                        },
                    );
                }
            }
        }
    }
    {
        let mut d = x.data.write().unwrap();
        d.step = x.dw.step;
        d.height = x.dw.height;
        d.width = x.dw.width;
        d.parts = x.dw.parts.clone();
        d.pids = dw_pids;
    }
}
pub struct AddPartArgs<'a> {
    pub x: f64,
    pub y: f64,
    pub dx: f64,
    pub dy: f64,
    pub data: &'a mut Data,
}
pub fn add_part(x: &mut AddPartArgs) {
    let i: u32 = (x.x % x.data.width).floor() as u32;
    let j: u32 = (x.y % x.data.height).floor() as u32;
    let part_id: u128 = Uuid::new_v4().as_u128();
    x.data.parts.insert(
        part_id,
        Part {
            x: x.x,
            y: x.y,
            x_old: x.x - x.dx,
            y_old: x.y - x.dy,
        },
    );
    match x.data.pids.get_mut(&(i, j)) {
        Some(pxy) => {
            pxy.insert(part_id);
        }
        None => {
            let mut pxy = HashSet::new();
            pxy.insert(part_id);
            x.data.pids.insert((i, j), pxy);
        }
    }
}
