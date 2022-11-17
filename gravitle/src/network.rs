use crate::Pid;
use crate::User;
use crate::Uuid;
use futures_channel::mpsc::{channel, Sender};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};
use std::collections::HashSet;
use std::sync::Arc;
use std::{collections::HashMap, net::SocketAddr, sync::Mutex};
use tokio::net::TcpStream;
use tungstenite::protocol::Message;
pub type Tx = Sender<Message>;
pub struct Peer {
    pub user_id: Option<u128>,
    pub addr: SocketAddr,
    pub tx: Tx,
}
pub struct NetworkData {
    pub peers: HashMap<SocketAddr, Peer>,
    pub users: HashMap<u128, User>,
    pub free_ship_pids: HashSet<Pid>,
}
pub type SharedNetworkData = Arc<Mutex<NetworkData>>;
pub async fn handle_connection(
    raw_stream: TcpStream,
    addr: SocketAddr,
    shared_data: SharedNetworkData,
) {
    println!("connecting {}", addr);
    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("connected {}", addr);
    let (tx, rx) = channel(0);
    shared_data.lock().unwrap().peers.insert(
        addr,
        Peer {
            tx,
            user_id: None,
            addr,
        },
    );
    let (outgoing, incoming) = ws_stream.split();
    let broadcast_incoming = incoming.try_for_each(|msg| {
        let msg_txt = msg.to_text().unwrap();
        println!("message from {}: {}", addr, msg.to_text().unwrap());
        if msg_txt.starts_with("request ship ") && msg_txt.len() == 13 + 36 {
            let uuid_str = &msg_txt.replace("request ship ", "");
            let uuid_u128 = Uuid::parse_str(uuid_str).unwrap().as_u128();
            match shared_data.lock() {
                Ok(mut data) => {
                    if !data.free_ship_pids.is_empty() {
                        println!("adding user {}", uuid_str);
                        let free_ship_pids_v: Vec<_> = data.free_ship_pids.iter().collect();
                        let pid = *free_ship_pids_v[0];
                        if !free_ship_pids_v.is_empty() {
                            // data.free_ship_pids.remove(&pid);
                            data.users.insert(
                                uuid_u128,
                                User {
                                    user_id: uuid_u128,
                                    addr,
                                    orders: HashMap::new(),
                                    ship_pid: pid,
                                },
                            );
                            data.peers.get_mut(&addr).unwrap().user_id = Some(uuid_u128);
                        }
                    }
                }
                Err(_) => {}
            }
        } else {
            let strs: Vec<&str> = msg_txt.split(' ').collect();
            if strs.len() == 2 {
                let pid: usize = strs[0].parse::<usize>().unwrap();
                let activation: f32 = strs[1].parse::<f32>().unwrap();
                match shared_data.lock() {
                    Ok(mut data) => match data.peers.get_mut(&addr).unwrap().user_id {
                        Some(user_id) => {
                            data.users
                                .get_mut(&user_id)
                                .unwrap()
                                .orders
                                .insert(pid, activation);
                        }
                        None => {}
                    },
                    Err(_) => {}
                }
            }
        }
        future::ok(())
    });
    let receive_from_others = rx.map(Ok).forward(outgoing);
    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;
    println!("disconnected {}", &addr);
    let ship_id = match shared_data.lock() {
        Ok(data) => match data.peers.get(&addr) {
            Some(peer) => match peer.user_id {
                Some(user_id) => data.users.get(&user_id).map(|user| user.ship_pid),
                None => None,
            },
            None => None,
        },
        Err(_) => None,
    };
    if let Some(ship_id) = ship_id {
                shared_data.lock().unwrap().free_ship_pids.insert(ship_id);
          };
    shared_data.lock().unwrap().peers.remove(&addr);
}
