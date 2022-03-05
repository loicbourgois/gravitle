use crate::data::Data;
use serde::{Deserialize, Serialize};
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::RwLock;
use std::time::Duration;
use tungstenite::Message;
use std::thread;
use tungstenite::accept;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use tungstenite::WebSocket;
use uuid::Uuid;
use std::collections::HashSet;

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

pub fn serve(senders: & Senders) {
    let server = TcpListener::bind("127.0.0.1:8000").unwrap();
    let senders_ = senders.clone();
    thread::spawn(move || {
        for stream in server.incoming() {
            println!("new stream");
            let senders = senders_.clone();
            thread::spawn(move || {
                let mut websocket = accept(stream.unwrap()).unwrap();
                let message = websocket.read_message().unwrap().to_string();
                match serde_json::from_str::<FirstMessage>(&message) {
                    first_message => match first_message {
                        Ok(m) => match m.request {
                            FirstMessageRequest::CreateSender => {
                                println!("new server sender");
                                senders.lock().unwrap().insert(m.uuid.as_u128(), websocket);
                            }
                            FirstMessageRequest::CreateReceiver => {
                                // println!("new server receiver");
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
    // parts: HashMap<u128, Part>,
    // pids: Vec<u128>,
    step: usize,
    // height: f32,
    // width: f32,
    // blocks: usize,
    // client_blocks: usize,
    // diameter: f64,
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

            // for data in datas.iter() {
            //     let dr = data.read().unwrap();
            //     // let mid = BLOCKS / 2;
            //     // let start_block = mid - CLIENT_BLOCKS / 2;
            //     // let end_block = mid + CLIENT_BLOCKS / 2;
            //     // let mut pids = Vec::new();
            //     // for i in start_block..end_block {
            //     //     for j in start_block..end_block {
            //     //         pids.append(&mut dr.pids[i][j].clone());
            //     //     }
            //     // }
            //     // data_client.step = dr.step;
            //     // for pid in &pids {
            //     //     data_client.parts.insert(*pid, dr.parts[&pid]);
            //     // }
            //     // data_client.pids.append(&mut pids);
            //     data_client.step = dr.step;
            // }
            let mut senders_to_delete: HashSet<u128> = HashSet::new();
            for (k, sender) in senders.lock().unwrap().iter_mut() {
                // match serde_json::to_string(&data_client) {
                //     Ok(d_string) => {
                let data_client: Vec<u8> = vec!(68;9);
                let m = Message::Binary(data_client);
                        match sender.write_message(m) {
                            Ok(_) => {}
                            Err(e) => {
                                println!("error A: {} {:?}", *k, e);
                                senders_to_delete.insert(*k);
                            }
                        };
                //     }
                //     Err(e) => {
                //         println!("error B: {:?}", e)
                //     }
                // }
            }
            for k in senders_to_delete.iter() {
                senders.lock().unwrap().remove(k);
            }
        thread::sleep(Duration::from_millis(5));
        }
    });
}
