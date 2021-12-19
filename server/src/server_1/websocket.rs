use crate::{
    server_1::server::{
        Data,
        BLOCKS,
        CLIENT_BLOCKS
    },
    part::Part
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    net::{TcpListener, TcpStream},
    sync::{
        Arc,
        Mutex,
        RwLock,
    },
    thread,
    time::Duration,
};
// use crate::main_hashmap2::{
//     BLOCKS,
//     CLIENT_BLOCKS
// };
use tungstenite::{accept, Message, WebSocket};
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
pub type Senders = Arc<Mutex<HashMap<u128, WebSocket<TcpStream>>>>;
pub struct ServeArgs<'a> {
    pub senders: &'a Senders,
}
pub fn serve(a: &ServeArgs) {
    let server = TcpListener::bind("127.0.0.1:8000").unwrap();
    let senders_ = a.senders.clone();
    thread::spawn(move || {
        for stream in server.incoming() {
            println!("new stream");
            let senders = senders_.clone();
            // let receivers_ = receivers.clone();
            thread::spawn(move || {
                let mut websocket = accept(stream.unwrap()).unwrap();
                let message = websocket.read_message().unwrap().to_string();
                match serde_json::from_str::<FirstMessage>(&message) {
                    first_message => match first_message {
                        Ok(m) => match m.request {
                            FirstMessageRequest::CreateSender => {
                                println!("new sender");
                                senders.lock().unwrap().insert(m.uuid.as_u128(), websocket);
                            }
                            FirstMessageRequest::CreateReceiver => {
                                println!("new receiver");
                                // receivers_
                                //     .lock()
                                //     .unwrap()
                                //     .insert(m.uuid.as_u128(), websocket);
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
}
#[derive(Serialize)]
struct DataClient {
    parts: HashMap<u128, Part>,
    pids: Vec<u128>,
    step: usize,
    height: f32,
    width: f32,
    blocks: usize,
    client_blocks: usize,
}
pub struct SendArgs<'a> {
    pub senders: &'a Senders,
    pub datas: &'a Vec<Arc<RwLock<Data>>>,
}
pub fn send(a: &SendArgs) {
    let datas = a.datas.clone();
    let senders = a.senders.clone();
    thread::spawn(move || {
        loop {
            let mut data_client = DataClient {
                step: 0,
                parts: HashMap::new(),
                pids: Vec::new(),
                height: BLOCKS as f32,
                width: BLOCKS as f32,
                blocks: BLOCKS,
                client_blocks: CLIENT_BLOCKS,
            };
            for data in datas.iter() {
                let dr = data.read().unwrap();
                let mid = BLOCKS/2;
                let start_block = mid - CLIENT_BLOCKS / 2;
                let end_block = mid +   CLIENT_BLOCKS / 2;
                let mut pids = Vec::new();
                for i in start_block..end_block {
                    for j in start_block..end_block {
                        let ij = i + j * BLOCKS;
                        pids.append(&mut dr.pids[ij].clone());
                    }
                }
                data_client.step = dr.step;
                for pid in &pids {
                    data_client.parts.insert(*pid, dr.parts[&pid]);
                }
                data_client.pids.append(&mut pids);
            }
            let mut senders_to_delete: HashSet<u128> = HashSet::new();
            for (k, sender) in senders.lock().unwrap().iter_mut() {
                match serde_json::to_string(&data_client) {
                    Ok(d_string) => {
                        match sender.write_message(Message::Text(d_string)) {
                            Ok(_) => {}
                            Err(e) => {
                                println!("error A: {} {:?}", *k, e);
                                senders_to_delete.insert(*k);
                            }
                        };
                    }
                    Err(e) => {
                        println!("error B: {:?}", e)
                    }
                }
            }
            for k in senders_to_delete.iter() {
                senders.lock().unwrap().remove(k);
            }
            thread::sleep(Duration::from_millis(10));
        }
    });
}
