use std::net::TcpListener;
use std::sync::RwLock;
use tungstenite::accept;
use rand;
use rand::Rng;
use std::{
    collections::{
        HashMap,
        HashSet
    },
    sync::{
        Arc,
        Mutex
    },
    thread,
    time::{
        Duration,
        SystemTime,
    }
};
use serde::{
    Serialize,
    Deserialize
};
use tungstenite::{
     Message,
     WebSocket
};
use uuid::Uuid;
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
enum FirstMessageRequest {
    CreateSender,
    CreateReceiver
}
#[derive(Serialize, Deserialize, Debug)]
struct FirstMessage {
    request : FirstMessageRequest,
    uuid: Uuid
}
use std::net::TcpStream;
type Senders = Arc<Mutex<HashMap<u128, WebSocket<TcpStream>>>>;
type Receivers = Arc<Mutex<HashMap<u128, WebSocket<TcpStream>>>>;
struct Data {
    step: usize,
    parts: [[Part; 256]; 256],
    width: f64,
    height: f64
}
#[derive(Serialize, Deserialize, Debug)]
struct DataClient {
    step: usize,
    parts: Vec<Vec<Part>>,
    width: f64,
    height: f64
}
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
struct Part {
    x: f64,
    y: f64,
    x_old: f64,
    y_old: f64,
}
fn main () {
    let server = TcpListener::bind("127.0.0.1:8000").unwrap();
    let senders: Senders = Arc::new(Mutex::new(HashMap::new()));
    let receivers: Receivers = Arc::new(Mutex::new(HashMap::new()));
    let data = Arc::new(RwLock::new( Data {
        step: 0,
        parts: [[Part{
            x: 0.0,
            y: 0.0,
            x_old: 0.0,
            y_old: 0.0,
        }; 256]; 256],
        width: 256.0,
        height: 256.0,
    }));
    // {
    //     let data_ = data.clone();
    //     let senders_ = senders.clone();
    //     thread::spawn (move || {
    //         loop {
    //             let d = {
    //                 let d = data_.read().unwrap();
    //                 Data {
    //                     step: d.step,
    //                     parts: d.parts,
    //                     // pids: d.pids,
    //                     height: d.height,
    //                     width: d.width,
    //                 }
    //             };
    //             let mut data_client = DataClient {
    //                 step: d.step,
    //                 parts: Vec::new(),
    //                 height: d.height,
    //                 width: d.width,
    //             };
    //             // for ( (i,j), pids) in d.pids.iter() {
    //             //     match data_client.pids.get_mut( i ) {
    //             //         Some(px) => {
    //             //             match px.get_mut( j ) {
    //             //                 Some(_) => {
    //             //                 },
    //             //                 None => {
    //             //                     px.insert(*j, pids.clone());
    //             //                 }
    //             //             }
    //             //         },
    //             //         None => {
    //             //             let mut pxy = HashMap::new();
    //             //             pxy.insert(*j, pids.clone());
    //             //             data_client.pids.insert(*i, pxy);
    //             //         }
    //             //     }
    //             // }
    //             let mut sender_to_delete: HashSet<u128> = HashSet::new();
    //             for (_k, sender) in senders_.lock().unwrap().iter_mut() {
    //                 match serde_json::to_string(&data_client) {
    //                     Ok(d_string) => {
    //                         match sender.write_message(Message::Text(
    //                             d_string
    //                         )) {
    //                             Ok(_) => {
    //
    //                             },
    //                             Err(e) => {
    //                                 println!("error A: {} {:?}", *_k, e);
    //                                 sender_to_delete.insert(*_k);
    //                             }
    //                         };
    //                     },
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
}
