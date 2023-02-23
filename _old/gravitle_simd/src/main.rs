#![feature(portable_simd)]
#![feature(test)]
extern crate test;
use std::simd::f32x16;
mod grid;
mod particle;
mod test_grid;
mod test_main;
mod test_particle;
mod test_world;
mod world;
use crate::grid::Grid;
use crate::particle::new_particles;
use crate::particle::Particle;
use crate::particle::Particles;
use crate::particle::Vector;
use crate::world::World;
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
use std::sync::RwLock;

pub fn wait<T: std::cmp::PartialOrd>(subsyncers: &Vec<Arc<RwLock<T>>>, i: usize) {
    loop {
        let mut ok = true;
        for s in subsyncers {
            if *(subsyncers[i].read().unwrap()) > *(s.read().unwrap()) {
                ok = false;
                break;
            }
        }
        if ok {
            break;
        }
        // thread::sleep(Duration::from_millis(1));
    }
}

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
    const THREAD_COUNT: usize = 2;
    let mut world = World::new(0.001, 50_000, 500, THREAD_COUNT);
    let mut syncers = Vec::new();
    for ph in 0..10 {
        let mut aa = Vec::new();
        for i in 0..THREAD_COUNT + 1 {
            aa.push(Arc::new(RwLock::new(0)));
        }
        syncers.push(aa)
    }
    for i in 0..THREAD_COUNT {
        let syncers = syncers.clone();
        let particles = world.particles.clone();
        let deltas = world.particle_deltas.clone();
        let grid = world.grid.clone();
        thread::spawn(move || {
            let mut rng = rand::thread_rng();
            loop {
                {
                    let mut w = syncers[0][i].write().unwrap();
                    *w += 1;
                }
                wait(&syncers[0], i);
                {
                    let mut w = syncers[1][i].write().unwrap();
                    World::update_01(
                        &mut particles[i].write().unwrap(),
                        &mut deltas[i].write().unwrap(),
                    );
                    *w += 1;
                    // println!("  #{} - update_01 - {}", i, *w);
                }
                wait(&syncers[1], i);

                {
                    let mut w = syncers[2][i].write().unwrap();
                    World::update_02(
                        &particles,
                        &mut deltas[i].write().unwrap(),
                        &grid.read().unwrap(),
                        world.particle_diameter_sqrd,
                    );
                    *w += 1;
                    // println!("  #{} - update_02 - {}", i, *w);
                }
                wait(&syncers[2], i);
                {
                    let mut w = syncers[3][i].write().unwrap();
                    World::update_03(&mut particles[i].write().unwrap(), &deltas);
                    *w += 1;
                    // println!("  #{} - update_03 - {}", i, *w);
                }
                wait(&syncers[3], i);
                {
                    let mut w = syncers[4][i].write().unwrap();
                    *w += 1;
                }
                wait(&syncers[4], i);
            }
        });
    }
    thread::spawn(move || {
        let mut last = Instant::now();
        let mut elapsed_compute_total = 0;
        loop {
            let start = Instant::now();
            {
                let mut w = syncers[0][THREAD_COUNT].write().unwrap();
                world.update_00();
                *w += 1;
                // println!("Main - update_00 - {}", *w);
            }
            wait(&syncers[0], THREAD_COUNT);
            {
                let mut w = syncers[1][THREAD_COUNT].write().unwrap();
                *w += 1;
            }
            wait(&syncers[1], THREAD_COUNT);
            {
                let mut w = syncers[2][THREAD_COUNT].write().unwrap();
                *w += 1;
            }
            wait(&syncers[2], THREAD_COUNT);
            {
                let mut w = syncers[3][THREAD_COUNT].write().unwrap();
                *w += 1;
            }
            wait(&syncers[3], THREAD_COUNT);
            {
                let mut w = syncers[4][THREAD_COUNT].write().unwrap();
                world.update_04();
                *w += 1;
            }
            wait(&syncers[4], THREAD_COUNT);
            let elapsed_compute = start.elapsed().as_micros();
            elapsed_compute_total += elapsed_compute;
            let elapsed = last.elapsed().as_micros();
            last = Instant::now();
            let start_data = Instant::now();
            let mut data = vec![];
            data.extend((world.step as u32).to_be_bytes().to_vec());
            data.extend((elapsed as u32).to_be_bytes().to_vec());
            data.extend((elapsed_compute as u32).to_be_bytes().to_vec());
            data.extend((elapsed_compute_total as u32).to_be_bytes().to_vec());
            data.extend((world.collissions as u32).to_be_bytes().to_vec());
            data.extend((world.particle_diameter).to_be_bytes().to_vec());
            data.extend((world.particle_count as u32).to_be_bytes().to_vec());
            for tparticles in &*world.particles {
                for particle in tparticles.read().unwrap().iter() {
                    data.extend((particle.p.x).to_be_bytes().to_vec());
                    data.extend((particle.p.y).to_be_bytes().to_vec());
                    data.extend((particle.collisions).to_be_bytes().to_vec());
                    data.extend((particle.thid as u32 ).to_be_bytes().to_vec());
                    data.extend((particle.fidx as u32 ).to_be_bytes().to_vec());
                }
            }
            // println!("elapsed data: {}", start_data.elapsed().as_micros());
            let start_send = Instant::now();
            let m = Message::Binary(data);
            for x in peers_2.lock().unwrap().values() {
                x.unbounded_send(m.clone()).unwrap();
            }
            // println!("send data: {}", start_send.elapsed().as_micros());

            // println!("elapsed short:   {}", start.elapsed().as_micros());
            // println!("elapsed full:    {}", elapsed);
            println!("avg elapsed compute: {}", elapsed_compute_total / world.step as u128);
            thread::sleep(Duration::from_millis(1));
        }
    });
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(peers.clone(), stream, addr));
    }
    Ok(())
}
