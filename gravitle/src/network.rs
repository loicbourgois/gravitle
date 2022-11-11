use crate::User;
use crate::Users;
use crate::Uuid;
use futures_channel::mpsc::{channel, Sender};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};
use std::sync::Arc;
use std::{collections::HashMap, net::SocketAddr, sync::Mutex};
use tokio::net::{TcpStream};
use tungstenite::protocol::Message;
// use std::collections::HashMap;
pub type Tx = Sender<Message>;
pub struct Peer {
    pub user_id: Option<u128>,
    pub addr: SocketAddr,
    pub tx: Tx,
}
pub type Peers = Arc<Mutex<HashMap<SocketAddr, Peer>>>;
pub async fn handle_connection(
    peers: Peers,
    raw_stream: TcpStream,
    addr: SocketAddr,
    users: Users,
) {
    println!("connecting {}", addr);
    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("connected {}", addr);
    let (tx, rx) = channel(0);
    peers.lock().unwrap().insert(
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
        // println!("message from {}: {}", addr, msg.to_text().unwrap());
        if msg_txt.starts_with("request ship ") && msg_txt.len() == 13 + 36 {
            let uuid_str = &msg_txt.replace("request ship ", "");
            let uuid_u128 = Uuid::parse_str(uuid_str).unwrap().as_u128();
            println!("adding user {}", uuid_str);
            users.lock().unwrap().insert(
                uuid_u128,
                User {
                    user_id: uuid_u128,
                    addr,
                    orders: HashMap::new(),
                },
            );
            peers.lock().unwrap().get_mut(&addr).unwrap().user_id = Some(uuid_u128);
        } else {
            let strs: Vec<&str> = msg_txt.split(" ").collect();
            if strs.len() == 2 {
                let pid:usize = strs[0].parse::<usize>().unwrap();
                let activation:f32 = strs[1].parse::<f32>().unwrap();
                // println!("{} {}", pid, activation);
                let user_id = peers.lock().unwrap().get_mut(&addr).unwrap().user_id.unwrap();
                users.lock().unwrap().get_mut(&user_id).unwrap().orders.insert(pid, activation);
            }
        }
        future::ok(())
    });
    let receive_from_others = rx.map(Ok).forward(outgoing);
    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;
    println!("disconnected {}", &addr);
    peers.lock().unwrap().remove(&addr);
}
