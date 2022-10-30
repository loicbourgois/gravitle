#![feature(portable_simd)]
#![feature(test)]
extern crate test;
use std::simd::f32x16;
use test::Bencher;
mod grid;
mod particle;
mod test_grid;
mod test_main;
mod test_particle;
mod test_world;
mod world;
use crate::grid::grid_id_particle;
use crate::grid::Grid;
use crate::particle::distance;
use crate::particle::new_particles;
use crate::particle::wrap_around;
use crate::particle::Particle;
use crate::particle::Particles;
use crate::particle::Vector;
use crate::particle::WrapAroundResponse;
use crate::world::World;
use rand::Rng;
use std::collections::HashSet;
fn add(a: &mut f32x16, b: &f32x16) {
    *a = *a + b;
}
fn add2(a: &mut [f32], b: &[f32]) {
    for i in 0..16 {
        a[i] = a[i] + b[i];
    }
}
use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};
use std::{
    collections::HashMap,
    env,
    io::Error as IoError,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::net::{TcpListener, TcpStream};
use tungstenite::protocol::Message;
type Tx = UnboundedSender<Message>;
type Peers = Arc<Mutex<HashMap<SocketAddr, Tx>>>;
async fn handle_connection(peers: Peers, raw_stream: TcpStream, addr: SocketAddr) {
    println!("Incoming TCP connection from: {}", addr);
    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);
    let (tx, rx) = unbounded();
    peers.lock().unwrap().insert(addr, tx);
    let (outgoing, incoming) = ws_stream.split();
    let broadcast_incoming = incoming.try_for_each(|msg| {
        println!(
            "Received a message from {}: {}",
            addr,
            msg.to_text().unwrap()
        );
        future::ok(())
    });
    let receive_from_others = rx.map(Ok).forward(outgoing);
    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;
    println!("{} disconnected", &addr);
    peers.lock().unwrap().remove(&addr);
}
use std::thread;
use std::time::Duration;
use std::time::Instant;
// https://docs.rs/rayon/latest/rayon/ ??
// bincode::serialize ??
#[tokio::main]
async fn main() -> Result<(), IoError> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());
    let peers = Peers::new(Mutex::new(HashMap::new()));
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);
    let peers_2 = peers.clone();
    let mut world = World::new(0.001, 50_000, 200);
    thread::spawn(move || {
        let mut last = Instant::now();
        loop {
            let elapsed = last.elapsed().as_micros();
            last = Instant::now();
            world.update();
            let mut data = vec![std::mem::size_of::<usize>() as u8];
            data.extend( world.step.to_be_bytes().to_vec() );
            data.extend( (elapsed as u32).to_be_bytes().to_vec() );
            let m = Message::Binary(data);
            for x in peers_2.lock().unwrap().values() {
                x.unbounded_send(m.clone()).unwrap();
            }
        }
    });
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(peers.clone(), stream, addr));
    }
    Ok(())
}
