#![deny(warnings)]
use rand;
use rand::Rng;
use std::sync::RwLock;
const TIMES_COUNT: usize = 100;
const SIZE: usize = 100;
const BASE_CAPACITY: usize = 10;
const COUNT: i32 = 50_000;
const THREADS: usize = 2;
use crate::part::Part;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::Arc,
    //thread,
    time::SystemTime,
    //    net::TcpListener
    //net::TcpStream;
};
//use tungstenite::{Message, WebSocket, accept};
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
    let data1s: Vec<Arc<RwLock<Data>>> = vec![Arc::new(RwLock::new(Data {
        step: 0,
        parts: HashMap::new(),
        pids: vec![vec![Vec::with_capacity(BASE_CAPACITY); SIZE]; SIZE],
        width: SIZE as f64,
        height: SIZE as f64,
    })); THREADS];
    let data2s: Vec<Arc<RwLock<Data>>> = vec![Arc::new(RwLock::new(Data {
        step: 0,
        parts: HashMap::new(),
        pids: vec![vec![Vec::with_capacity(BASE_CAPACITY); SIZE]; SIZE],
        width: SIZE as f64,
        height: SIZE as f64,
    })); THREADS];
    for (i, data1) in data1s.iter().enumerate() {
        init(&data1);
        let mut dw = data2s[i].write().unwrap();
        let dr = data1.read().unwrap();
        dw.step = dr.step;
        dw.parts= dr.parts.clone();
        dw.pids = dr.pids.clone();
        dw.width = dr.width;
        dw.height =  dr.height;
    }
    compute_loop(&data1s[0], &data2s[0]);
    // {
    //     let data_ = data.clone();
    //     let senders_ = senders.clone();
    //     thread::spawn(move || {
    //         loop {
    //             let d = {
    //                 let d = data_.read().unwrap();
    //                 Data {
    //                     step: d.step,
    //                     parts: d.parts.clone(),
    //                     pids: d.pids.clone(),
    //                     height: d.height,
    //                     width: d.width,
    //                 }
    //             };
    //             let mut data_client = DataClient {
    //                 step: d.step,
    //                 // parts: d.parts.clone(),
    //                 parts: HashMap::new(),
    //                 pids: vec![vec![HashSet::new(); 500]; 500],
    //                 height: d.height,
    //                 width: d.width,
    //             };
    //             for ((i, j), pids) in d.pids.iter() {
    //                 if ((*i as i32) - 250).abs() < 30 && ((*j as i32) - 250).abs() < 30 {
    //                     //data_client.pids[*i as usize][*j as usize] = pids.clone();
    //                     for pid in pids {
    //                         let part = d.parts.get(pid).unwrap();
    //                         data_client.parts.insert(*pid, *part);
    //                     }
    //                 }
    //             }
    //             let mut sender_to_delete: HashSet<u128> = HashSet::new();
    //             for (_k, sender) in senders_.lock().unwrap().iter_mut() {
    //                 match serde_json::to_string(&data_client) {
    //                     Ok(d_string) => {
    //                         match sender.write_message(Message::Text(d_string)) {
    //                             Ok(_) => {}
    //                             Err(e) => {
    //                                 println!("error A: {} {:?}", *_k, e);
    //                                 sender_to_delete.insert(*_k);
    //                             }
    //                         };
    //                     }
    //                     Err(e) => {
    //                         println!("error B: {:?}", e)
    //                     }
    //                 }
    //             }
    //             for k in sender_to_delete.iter() {
    //                 senders_.lock().unwrap().remove(k);
    //             }
    //             thread::sleep(Duration::from_millis(10));
    //         }
    //     });
    // }
    // thread::spawn(move || {
    //     for stream in server.incoming() {
    //         println!("new stream");
    //         let senders_ = senders.clone();
    //         let receivers_ = receivers.clone();
    //         thread::spawn(move || {
    //             let mut websocket = accept(stream.unwrap()).unwrap();
    //             let message = websocket.read_message().unwrap().to_string();
    //             match serde_json::from_str::<FirstMessage>(&message) {
    //                 first_message => match first_message {
    //                     Ok(m) => match m.request {
    //                         FirstMessageRequest::CreateSender => {
    //                             senders_.lock().unwrap().insert(m.uuid.as_u128(), websocket);
    //                         }
    //                         FirstMessageRequest::CreateReceiver => {
    //                             receivers_
    //                                 .lock()
    //                                 .unwrap()
    //                                 .insert(m.uuid.as_u128(), websocket);
    //                         }
    //                     },
    //                     Err(e) => {
    //                         println!("{:?}", e)
    //                     }
    //                 },
    //             }
    //         });
    //     }
    // });
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
fn compute_loop(data1: &Arc<RwLock<Data>>, data2: &Arc<RwLock<Data>>) {
    let mut times: Vec<SystemTime> = Vec::new();
    let mut step = 0;
    let mut data = ComputeArgs {
        times: &mut times,
        step: &mut step,
        data1: &data1,
        data2: &data2,
    };
    loop {
        compute(&mut data);
    }
}
pub struct ComputeArgs<'a> {
    pub times: &'a mut Vec<SystemTime>,
    pub data1: &'a Arc<RwLock<Data>>,
    pub data2: &'a Arc<RwLock<Data>>,
    pub step: &'a mut usize,
}
fn compute(arg: &mut ComputeArgs) {
    let (dr, mut dw) = {
        if *arg.step % 2 == 0 {
            (arg.data1.read().unwrap(), arg.data2.write().unwrap())
        } else {
            (arg.data2.read().unwrap(), arg.data1.write().unwrap())
        }
    };
    dw.step = dr.step + 1;
    *arg.step = dw.step;
    arg.times.push(SystemTime::now());
    if (dw.step % 10) == 0 {
        println!("step: {}", dr.step);
        if arg.times.len() > TIMES_COUNT {
            arg.times.drain(0..1);
            let compute = arg.times[TIMES_COUNT - 1]
                .duration_since(arg.times[0])
                .unwrap()
                / TIMES_COUNT.try_into().unwrap();
            println!("cps:      {:?}", 1.0 / compute.as_secs_f32());
            println!("compute:  {:?}", compute);
        }
    }
    let mut dw_pids: Vec<Vec<Vec<u128>>> =
        vec![vec![Vec::with_capacity(BASE_CAPACITY); SIZE]; SIZE];
    for a in dr.pids.iter() {
        for pids in a.iter() {
            for pid in pids.iter() {
                let p1r = dr.parts.get(pid).unwrap();
                let dx = p1r.x - p1r.x_old;
                let dy = p1r.y - p1r.y_old;
                let x = (p1r.x + dx) % dr.width;
                let y = (p1r.y + dy) % dr.height;
                let i: usize = (x % dr.width).floor() as usize;
                let j: usize = (y % dr.height).floor() as usize;
                dw_pids[i][j].push(*pid);
                match dw.parts.get_mut(pid) {
                    Some(p) => {
                        *p = Part {
                            x: x,
                            y: y,
                            x_old: x - dx,
                            y_old: y - dy,
                        };
                    }
                    None => {
                        dw.parts.insert(
                            *pid,
                            Part {
                                x: x,
                                y: y,
                                x_old: x - dx,
                                y_old: y - dy,
                            },
                        );
                    }
                }
            }
        }
    }
    dw.pids = dw_pids;
}
pub struct AddPartArgs<'a> {
    pub x: f64,
    pub y: f64,
    pub dx: f64,
    pub dy: f64,
    pub data: &'a mut Data,
}
pub fn add_part(x: &mut AddPartArgs) {
    let i: usize = (x.x % x.data.width).floor() as usize;
    let j: usize = (x.y % x.data.height).floor() as usize;
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
    x.data.pids[i][j].push(part_id)
}
