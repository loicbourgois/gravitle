use crate::grid::grid_id_position;
use crate::grid::Grid;
use crate::grid::GridConfiguration;
use crate::math::wrap_around;
use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;
use std::time::Instant;
mod grid;
mod math;
mod particle;
use crate::particle::Particle;
use crate::particle::ParticleConfiguration;
pub struct Configuration {
    pub particle_count: usize,
    pub thread_count: usize,
    pub diameter: f32,
}
#[derive(Clone, Copy)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}
struct Delta {
    collisions: u32,
    pid: usize, // particle id
    tid: usize, // thread id
    dtid: usize,
    did: usize,
}
pub struct World {
    pub particle_count: usize,
    pub thread_count: usize,
    pub diameter: f32,
    pub particle_per_thread: usize,
    pub particle_diameter_sqrd: f32,
}
impl World {
    pub fn new(c: &Configuration) -> World {
        World {
            particle_count: c.particle_count,
            thread_count: c.thread_count,
            diameter: c.diameter,
            particle_per_thread: c.particle_count / c.thread_count,
            particle_diameter_sqrd: c.diameter * c.diameter,
        }
    }
}
pub fn neighbours<'a>(position: &'a Vector, grid: &'a Grid) -> [&'a Vec<usize>; 9] {
    let gid = grid_id_position(position, grid.side);
    [
        &grid.pids[grid.gids[gid][0]],
        &grid.pids[grid.gids[gid][1]],
        &grid.pids[grid.gids[gid][2]],
        &grid.pids[grid.gids[gid][3]],
        &grid.pids[grid.gids[gid][4]],
        &grid.pids[grid.gids[gid][5]],
        &grid.pids[grid.gids[gid][6]],
        &grid.pids[grid.gids[gid][7]],
        &grid.pids[grid.gids[gid][8]],
    ]
}
pub fn wait(subsyncers: &Vec<Arc<RwLock<usize>>>, i: usize) {
    loop {
        let mut ok = true;
        for s in subsyncers {
            let a = *(subsyncers[i].read().unwrap());
            let b = *(s.read().unwrap());
            if a > b || a < b - 1 {
                ok = false;
                break;
            }
        }
        if ok {
            break;
        }
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
    let world = World::new(&Configuration {
        particle_count: 100_000,
        thread_count: 5,
        diameter: 0.001,
    });
    let mut grid = Grid::new(&GridConfiguration { side: 1000 });
    let mut particles = Vec::new();
    let mut deltas = Vec::new();
    // let mut rng = rand::thread_rng();
    for dtid in 0..world.thread_count {
        for pid in 0..world.particle_count {
            let tid = pid % world.thread_count;
            assert!(deltas.len() == dtid * world.particle_count + pid);
            deltas.push(Delta {
                collisions: 0,
                pid,
                tid,
                dtid,
                did: deltas.len(),
            });
        }
    }
    for pid in 0..world.particle_count {
        particles.push(Particle::new(&ParticleConfiguration { pid, world: &world }));
    }
    for tid in 0..world.thread_count {
        for i in 0..world.particle_per_thread {
            let pid = i * world.thread_count + tid;
            assert!(particles[pid].pid == pid);
            assert!(particles[pid].tid == tid);
        }
    }
    assert!(deltas.len() == world.particle_count * world.thread_count);
    for did in 0..deltas.len() {
        let delta = &deltas[did];
        assert!(delta.did == delta.dtid * world.particle_count + delta.pid);
        assert!(delta.pid == did % world.particle_count);
        assert!(delta.tid == (did % world.particle_count) % world.thread_count);
        assert!(delta.dtid == did / world.particle_count);
        assert!(delta.did == did);
        assert!(particles[delta.pid].pid == delta.pid);
        assert!(particles[delta.pid].tid == delta.tid);
    }
    let mut threads = Vec::new();
    let mut syncers = Vec::new();
    for _ in 0..4 {
        let mut subsyncers = Vec::new();
        for _ in 0..world.thread_count + 1 {
            subsyncers.push(Arc::new(RwLock::new(0)));
        }
        syncers.push(subsyncers)
    }
    for tid in 0..world.thread_count {
        let particles_ptr = AtomicPtr::new(&mut particles);
        let deltas_ptr = AtomicPtr::new(&mut deltas);
        let grid_ptr = AtomicPtr::new(&mut grid);
        let syncers = syncers.clone();
        threads.push(thread::spawn(move || {
            unsafe {
                let particles = &mut (*particles_ptr.load(Ordering::Relaxed));
                let deltas = &mut (*deltas_ptr.load(Ordering::Relaxed));
                let particles2 = &mut (*particles_ptr.load(Ordering::Relaxed));
                let grid = &(*grid_ptr.load(Ordering::Relaxed));
                loop {
                    {
                        let mut w = syncers[0][tid].write().unwrap();
                        *w += 1;
                    }
                    wait(&syncers[0], tid);
                    //
                    // Read particles
                    // Write deltas
                    //
                    {
                        let mut w = syncers[1][tid].write().unwrap();
                        for i in 0..world.particle_per_thread {
                            let pid1 = i * world.thread_count + tid;
                            let mut p1 = &mut particles[pid1];
                            p1.collisions = 0;
                            for ns in neighbours(&p1.p, grid) {
                                for pid2 in ns {
                                    let p2 = &mut (*particles2)[*pid2];
                                    if p1.pid < p2.pid {
                                        let wa = wrap_around(&p1.p, &p2.p);
                                        if wa.d_sqrd < world.particle_diameter_sqrd {
                                            let did1 = tid * world.particle_count + p1.pid;
                                            deltas[did1].collisions += 1;
                                            let did2 = tid * world.particle_count + p2.pid;
                                            deltas[did2].collisions += 1;
                                        }
                                    }
                                }
                            }
                        }
                        *w += 1;
                    }
                    wait(&syncers[1], tid);
                    //
                    // Read deltas
                    // Write particles
                    //
                    {
                        let mut w = syncers[2][tid].write().unwrap();
                        for i in 0..world.particle_per_thread {
                            let pid1 = i * world.thread_count + tid;
                            let mut p1 = &mut particles[pid1];
                            for tid in 0..world.thread_count {
                                let did1 = tid * world.particle_count + p1.pid;
                                p1.collisions += deltas[did1].collisions;
                                deltas[did1].collisions = 0;
                                p1.v.x = p1.p.x - p1.pp.x;
                                p1.v.y = p1.p.y - p1.pp.y;
                                p1.p.x = (1.0 + p1.p.x + p1.v.x) % 1.0;
                                p1.p.y = (1.0 + p1.p.y + p1.v.y) % 1.0;
                                p1.pp.x = p1.p.x - p1.v.x;
                                p1.pp.y = p1.p.y - p1.v.y;
                            }
                        }
                        *w += 1;
                    }
                    //
                    //
                    //
                    //
                    wait(&syncers[2], tid);
                    {
                        let mut w = syncers[3][tid].write().unwrap();
                        *w += 1;
                    }
                    wait(&syncers[3], tid);
                }
            }
        }));
    }
    let mut elapsed_total = 0;
    let mut step = 0;
    {
        let peers = peers.clone();
        thread::spawn(move || loop {
            let start = Instant::now();
            {
                let mut w = syncers[0][world.thread_count].write().unwrap();
                grid.update_01();
                grid.update_02(&mut particles);
                *w += 1;
            }
            wait(&syncers[0], world.thread_count);
            {
                let mut w = syncers[1][world.thread_count].write().unwrap();
                *w += 1;
            }
            wait(&syncers[1], world.thread_count);
            {
                let mut w = syncers[2][world.thread_count].write().unwrap();
                *w += 1;
            }
            wait(&syncers[2], world.thread_count);
            {
                let mut w = syncers[3][world.thread_count].write().unwrap();
                let mut collisions_count = 0;
                for p in &particles {
                    collisions_count += p.collisions;
                }
                let elapsed_compute = start.elapsed().as_micros();
                let capacity = 3 * 4 * world.particle_count + 7 * 4;
                let mut data = Vec::with_capacity(capacity);
                data.extend((step as u32).to_be_bytes().to_vec());
                data.extend((elapsed_total as u32).to_be_bytes().to_vec());
                data.extend((elapsed_compute as u32).to_be_bytes().to_vec());
                data.extend((elapsed_total as u32).to_be_bytes().to_vec());
                data.extend(collisions_count.to_be_bytes().to_vec());
                data.extend((world.diameter).to_be_bytes().to_vec());
                data.extend((world.particle_count as u32).to_be_bytes().to_vec());
                let mut data_2: Vec<u8> = vec![0; 3 * 4 * world.particle_count];
                for pid in 0..particles.len() {
                    let i = pid * 3 * 4;
                    let xs = particles[pid].p.x.to_be_bytes();
                    let ys = particles[pid].p.y.to_be_bytes();
                    let cs = particles[pid].collisions.to_be_bytes();
                    data_2[i..(4 + i)].copy_from_slice(&xs[..4]);
                    data_2[(4 + i)..(8 + i)].copy_from_slice(&ys[..4]);
                    data_2[(8 + i)..(12 + i)].copy_from_slice(&cs[..4]);
                    // for j in 0..4 {
                    //     data_2[i + j] = xs[j];
                    // }
                    //
                    // for j in 0..4 {
                    //     data_2[i + j + 4] = ys[j];
                    // }
                    //
                    // for j in 0..4 {
                    //     data_2[i + j + 8] = cs[j];
                    // }
                }
                data.extend(data_2);
                assert!(data.len() == capacity);
                let m = Message::Binary(data);
                for x in peers.lock().unwrap().values() {
                    x.unbounded_send(m.clone()).unwrap();
                }
                *w += 1;
            }
            wait(&syncers[3], world.thread_count);
            elapsed_total += start.elapsed().as_micros();
            step += 1;
            let delta = Duration::from_millis(100);
            if start.elapsed() < delta {
                let sleep_duration = delta - start.elapsed();
                thread::sleep(sleep_duration);
            }
        });
    }
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(peers.clone(), stream, addr));
    }
    Ok(())
}
use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};
use std::{collections::HashMap, env, io::Error as IoError, net::SocketAddr, sync::Mutex};
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
