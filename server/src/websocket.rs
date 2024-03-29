use crate::data::Data;
use crate::gravitle::cell_id;
use crate::gravitle::part_id;
use crate::gravitle::HEIGHT;
use crate::gravitle::WIDTH;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;
use tungstenite::accept;
use tungstenite::Message;
use tungstenite::WebSocket;
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

pub fn serve(senders: &Senders) {
    let server = TcpListener::bind("0.0.0.0:8000").unwrap();
    let senders_ = senders.clone();
    thread::spawn(move || {
        for stream in server.incoming() {
            println!("new stream");
            let senders = senders_.clone();
            thread::spawn(move || {
                let stream = match stream {
                    Ok(stream) => stream,
                    Err(e) => panic!("[ error ] can not get stream: {}", e),
                };
                let mut websocket = match accept(stream) {
                    Ok(websocket) => websocket,
                    Err(e) => panic!("[ error ] can not get websocket: {}", e),
                };
                let message = websocket.read_message().unwrap().to_string();

                let first_message = serde_json::from_str::<FirstMessage>(&message);

                match first_message {
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
                }
            });
        }
    });
}
use core::data_client::DataClient;

pub struct SendArgs<'a> {
    pub senders: &'a Senders,
    pub datas: &'a Vec<Arc<RwLock<Data>>>,
}
pub fn send(a: &SendArgs) {
    let datas = a.datas.clone();
    let senders = a.senders.clone();
    thread::spawn(move || loop {
        let mut senders_to_delete: HashSet<u128> = HashSet::new();
        for (k, sender) in senders.lock().unwrap().iter_mut() {
            let start_i = 0;
            let count_i = 100;
            let start_j = 0;
            let count_j = 100;
            let mut parts = Vec::new();
            let mut part_count: u32 = 0;
            for data in datas.iter() {
                let dr = data.read().unwrap();
                for i_ in start_i..start_i + count_i {
                    let i = i_ % WIDTH;
                    for j_ in start_j..start_j + count_j {
                        let j = j_ % HEIGHT;
                        let cid = cell_id(i, j);
                        let depth = dr.depths[cid];
                        part_count += depth as u32;
                        for k in 0..depth {
                            parts.push(dr.parts[part_id(cid, k)]);
                        }
                    }
                }
            }
            let data_client: Vec<u8> = bincode::serialize(&DataClient {
                step: datas[0].read().unwrap().step as u32,
                part_count,
                parts,
                width: WIDTH as u32,
                height: HEIGHT as u32,
                i_start: start_i as u32,
                i_size: count_i as u32,
                j_start: start_j as u32,
                j_size: count_j as u32,
            })
            .unwrap();
            let message = Message::Binary(data_client);
            match sender.write_message(message) {
                Ok(_) => {}
                Err(error) => {
                    println!("error A: {} {:?}", *k, error);
                    senders_to_delete.insert(*k);
                }
            };
        }
        for k in senders_to_delete.iter() {
            senders.lock().unwrap().remove(k);
        }
        thread::sleep(Duration::from_millis(30));
    });
}
