use crate::grid::grid_id_position;
use crate::grid::Grid;
use crate::grid::GridConfiguration;
use crate::math::collision_response;
use crate::math::normalize;
use crate::math::wrap_around;
use chrono::Utc;
use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;
use crate::math::normalize_2;
use std::time::Instant;
mod grid;
mod math;
mod network;
mod particle;
// use std::collections::HashMap;
mod test_math;
use crate::grid::grid_id;
use crate::grid::grid_xy;
use crate::network::handle_connection;
use crate::network::Peers;
use crate::particle::Particle;
use futures_channel::mpsc::{channel, Sender};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};
use std::{collections::HashMap, env, io::Error as IoError, net::SocketAddr, sync::Mutex};
use tokio::net::{TcpListener, TcpStream};
use tungstenite::protocol::Message;
use uuid::Uuid;
pub struct User {
    pub user_id: u128,
    pub addr: SocketAddr,
    pub orders: HashMap<usize, f32>,
}
type Users = Arc<Mutex<HashMap<u128, User>>>;
pub struct Configuration {
    pub particle_count: usize,
    pub thread_count: usize,
    pub diameter: f32,
}
#[derive(Clone, Copy, Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}
#[derive(Clone, Copy, Debug)]
pub struct Vector_u {
    pub x: usize,
    pub y: usize,
}
#[derive(Debug)]
struct Delta {
    collisions: u32,
    p: Vector,
    v: Vector,
    pid: usize, // particle id
    tid: usize, // thread id
    dtid: usize,
    did: usize,
    direction: Vector,
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
        .unwrap_or_else(|| "0.0.0.0:8000".to_string());
    let peers = Peers::new(Mutex::new(HashMap::new()));
    let users = Users::new(Mutex::new(HashMap::new()));
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);
    let world = World::new(&Configuration {
        particle_count: 50_000,
        thread_count: 5,
        diameter: 0.001 * 0.5 ,
    });
    let crd = 0.01;    // collision response delta (position)
    let crdv = 0.9;     // collision response delta (velocity)
    let LINK_STRENGH = 0.2;
    let linkt_length_ratio = 1.0;
    let booster_acceleration = 0.0001*0.05;
    let mut grid = Grid::new(&GridConfiguration { side: 1024 });
    assert!(1.0 / grid.side as f32 > world.diameter);
    let mut links: Vec<[usize;2]> = Vec::new();
    let setup = 5;
    let mut particles = Particle::new_particles_4(&world);
    if setup == 5 {
        particles = Particle::new_particles_5(&world);
        links.push([0, 1]);
        links.push([0, 2]);
        links.push([0, 3]);
        links.push([0, 4]);
        links.push([0, 5]);
        links.push([0, 6]);
        links.push([1, 2]);
        links.push([2, 3]);
        links.push([3, 4]);
        links.push([4, 5]);
        links.push([5, 6]);
        links.push([1, 6]);
        links.push([3, 8]);
        links.push([3, 7]);
        links.push([2, 7]);
        links.push([7, 8]);
        links.push([8, 9]);
        links.push([7, 9]);
        links.push([9, 10]);
        links.push([7, 10]);
        links.push([6, 11]);
        links.push([11, 12]);
        links.push([1, 11]);
        links.push([11, 13]);
        links.push([11, 14]);
        links.push([6, 12]);
        links.push([12, 13]);
        links.push([13, 14]);
    }
    for link in &links {
        assert!(link[0] < link[1]);
    }

    let mut deltas = Vec::new();
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
                p: Vector { x: 0.0, y: 0.0 },
                v: Vector { x: 0.0, y: 0.0 },
                direction: Vector { x: 0.0, y: 0.0 },
            });
        }
    }
    for tid in 0..world.thread_count {
        for i in 0..world.particle_per_thread {
            let pid = i * world.thread_count + tid;
            assert!(particles[pid].pid == pid);
            assert!(particles[pid].tid == tid);
        }
    }
    assert!(deltas.len() == world.particle_count * world.thread_count);
    for (did, delta) in deltas.iter().enumerate() {
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
        let links_ptr = AtomicPtr::new(&mut links);
        let deltas_ptr = AtomicPtr::new(&mut deltas);
        let grid_ptr = AtomicPtr::new(&mut grid);
        let syncers = syncers.clone();
        threads.push(thread::spawn(move || {
            unsafe {
                let particles = &mut (*particles_ptr.load(Ordering::Relaxed));
                let links = &mut (*links_ptr.load(Ordering::Relaxed));
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
                    // Read link
                    // Write deltas
                    //
                    {
                        let mut w = syncers[1][tid].write().unwrap();
                        for i in 0..world.particle_per_thread {
                            let pid1 = i * world.thread_count + tid;
                            let mut neigh_c = 0;
                            let mut p1 = &mut particles[pid1];
                            for ns in neighbours(&p1.p, grid) {
                                for pid2 in ns {
                                    let p2 = &mut (*particles2)[*pid2];
                                    if p1.pid < p2.pid {
                                        neigh_c += 1;
                                        let wa = wrap_around(&p1.p, &p2.p);
                                        if wa.d_sqrd < world.particle_diameter_sqrd {
                                            let cr = collision_response(&wa, p1, p2);
                                            if !cr.x.is_nan() && !cr.y.is_nan() {
                                                {
                                                    let d1 = &mut deltas
                                                        [tid * world.particle_count + p1.pid];
                                                    d1.collisions += 1;
                                                    d1.v.x += cr.x * crdv;
                                                    d1.v.y += cr.y * crdv;
                                                    d1.p.x -= wa.d.x * crd;
                                                    d1.p.y -= wa.d.y * crd;
                                                }
                                                {
                                                    let d2 = &mut deltas
                                                        [tid * world.particle_count + p2.pid];
                                                    d2.collisions += 1;
                                                    d2.v.x -= cr.x * crdv;
                                                    d2.v.y -= cr.y * crdv;
                                                    d2.p.x += wa.d.x * crd;
                                                    d2.p.y += wa.d.y * crd;
                                                }
                                            }
                                            //   if (links_set.has(`${p1.idx}|${p2.idx}`)) {
                                            //     cr.x *= 0.5;
                                            //     cr.y *= 0.5;
                                            //   }
                                        }
                                    }
                                }
                            }
                            // if (pid1 == 0) {
                            //     println!("{}", neigh_c);
                            // }
                        }

                        let size = links.len() / world.thread_count + 1;
                        let start = size * tid;
                        let end = (start + size).min(links.len());

                        for lid in start..end {
                            let p1 = &particles[links[lid][0]];
                            let p2 = &particles[links[lid][1]];
                            let wa = wrap_around(&p1.p, &p2.p);
                            let d = wa.d_sqrd.sqrt();
                            let n = normalize(&wa.d, d);
                            let factor = (world.diameter*linkt_length_ratio - d) * LINK_STRENGH;

                            if wa.d_sqrd < world.particle_diameter_sqrd {
                                let cr = collision_response(&wa, p1, p2);
                                if !cr.x.is_nan() && !cr.y.is_nan() {
                                    {
                                        let d1 = &mut deltas
                                            [tid * world.particle_count + p1.pid];
                                        d1.collisions += 1;
                                        d1.v.x -= cr.x * crdv * 0.5;
                                        d1.v.y -= cr.y * crdv * 0.5;
                                        d1.p.x += wa.d.x * crd;
                                        d1.p.y += wa.d.y * crd;
                                    }
                                    {
                                        let d2 = &mut deltas
                                            [tid * world.particle_count + p2.pid];
                                        d2.collisions += 1;
                                        d2.v.x += cr.x * crdv * 0.5;
                                        d2.v.y += cr.y * crdv * 0.5;
                                        d2.p.x -= wa.d.x * crd;
                                        d2.p.y -= wa.d.y * crd;
                                    }
                                }
                            }
                            if !n.x.is_nan() && !n.y.is_nan() {

                            {
                                let d1 = &mut deltas[tid * world.particle_count + p1.pid];
                                d1.v.x -= n.x * factor;
                                d1.v.y -= n.y * factor;
                                d1.direction.x += wa.d.x;
                                d1.direction.y += wa.d.y;
                            }
                            {
                                let d2 = &mut deltas[tid * world.particle_count + p2.pid];
                                d2.v.x += n.x * factor;
                                d2.v.y += n.y * factor;
                                d2.direction.x -= wa.d.x;
                                d2.direction.y -= wa.d.y;
                            }
                        }
                        }
                        *w += 1;
                    }
                    wait(&syncers[1], tid);
                    //
                    // Reset particle
                    // Read delta
                    // Reset delta
                    // Write particle
                    //
                    {
                        let mut w = syncers[2][tid].write().unwrap();
                        for i in 0..world.particle_per_thread {
                            let pid1 = i * world.thread_count + tid;
                            let mut p1 = &mut particles[pid1];
                            p1.collisions = 0;
                            p1.direction.x = 0.0;
                            p1.direction.y = 0.0;
                            p1.v.x = p1.p.x - p1.pp.x;
                            p1.v.y = p1.p.y - p1.pp.y;
                            for tid in 0..world.thread_count {
                                let d1 = &mut deltas[tid * world.particle_count + p1.pid];
                                // assert!(d1.p.x >= 0.0, "\n{:?}", d1);
                                // assert!(d1.p.y >= 0.0, "\n{:?}", d1);
                                // assert!(d1.p.x <= 1.0, "\n{:?}", d1);
                                // assert!(d1.p.y <= 1.0, "\n{:?}", d1);
                                assert!(!d1.v.x.is_nan(), "\n{:?}", d1);
                                assert!(!d1.v.y.is_nan(), "\n{:?}", d1);
                                assert!(!d1.p.x.is_nan(), "\n{:?}", d1);
                                assert!(!d1.p.y.is_nan(), "\n{:?}", d1);
                                p1.collisions += d1.collisions;
                                p1.direction.x += d1.direction.x;
                                p1.direction.y += d1.direction.y;
                                p1.v.x += d1.v.x;
                                p1.v.y += d1.v.y;
                                p1.p.x += d1.p.x;
                                p1.p.y += d1.p.y;
                                d1.collisions = 0;
                                d1.p.x = 0.0;
                                d1.p.y = 0.0;
                                d1.v.x = 0.0;
                                d1.v.y = 0.0;
                                d1.direction.x = 0.0;
                                d1.direction.y = 0.0;
                            }
                            p1.direction = normalize_2(&p1.direction);
                            if p1.direction.x.is_nan() || p1.direction.y.is_nan() {
                                p1.direction.x = 0.0;
                                p1.direction.y = 0.0;
                            }
                            p1.v.x += p1.direction.x * p1.activation * booster_acceleration;
                            p1.v.y += p1.direction.y * p1.activation * booster_acceleration;
                            p1.v.x = p1.v.x.max(-world.diameter * 0.5);
                            p1.v.x = p1.v.x.min(world.diameter * 0.5);
                            p1.v.y = p1.v.y.max(-world.diameter * 0.5);
                            p1.v.y = p1.v.y.min(world.diameter * 0.5);
                            p1.p.x = (10.0 + p1.p.x + p1.v.x) % 1.0;
                            p1.p.y = (10.0 + p1.p.y + p1.v.y) % 1.0;
                            p1.pp.x = p1.p.x - p1.v.x;
                            p1.pp.y = p1.p.y - p1.v.y;
                            assert!(!p1.p.x.is_nan(), "\n{:?}", p1);
                            assert!(!p1.p.y.is_nan(), "\n{:?}", p1);
                            assert!(p1.v.x >= -world.diameter, "\n{:?}", p1);
                            assert!(p1.v.y >= -world.diameter, "\n{:?}", p1);
                            assert!(p1.v.x <= world.diameter, "\n{:?}", p1);
                            assert!(p1.v.y <= world.diameter, "\n{:?}", p1);
                            assert!(p1.p.x >= 0.0, "\n{:?}", p1);
                            assert!(p1.p.y >= 0.0, "\n{:?}", p1);
                            assert!(p1.p.x <= 1.0, "\n{:?}", p1);
                            assert!(p1.p.y <= 1.0, "\n{:?}", p1);
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
        let users = users.clone();
        thread::spawn(move || loop {
            let start = Instant::now();
            {
                let mut w = syncers[0][world.thread_count].write().unwrap();
                grid.update_01();
                grid.update_02(&mut particles);
                for user in users.lock().unwrap().values_mut() {
                    for (pid, activation) in &user.orders {
                        particles[*pid].activation = *activation;
                    }
                    user.orders.clear();
                }
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
                let part_bytes = 2 + 2 + 1;
                let common_capacity = 4 * 9 + 8;
                let capacity = world.particle_count * part_bytes + common_capacity;
                let mut data = vec![0; capacity];
                let mut data_common = Vec::with_capacity(common_capacity);
                data_common.extend(Utc::now().timestamp_millis().to_be_bytes().to_vec());
                data_common.extend((step as f32).to_be_bytes().to_vec());
                data_common.extend((elapsed_total as f32).to_be_bytes().to_vec());
                data_common.extend((elapsed_compute as f32).to_be_bytes().to_vec());
                data_common.extend((elapsed_total as f32).to_be_bytes().to_vec());
                data_common.extend((peers.lock().unwrap().len() as u32).to_be_bytes().to_vec());
                data_common.extend(collisions_count.to_be_bytes().to_vec());
                data_common.extend((world.diameter).to_be_bytes().to_vec());
                data_common.extend((world.particle_count as u32).to_be_bytes().to_vec());
                data_common.extend(((256.0 * 256.0) as f32).to_be_bytes().to_vec());
                data[..common_capacity].copy_from_slice(&data_common);
                let _data_2: Vec<u8> = vec![0; part_bytes * world.particle_count];
                for (pid, particle) in particles.iter().enumerate() {
                    let i = common_capacity + pid * part_bytes;
                    let xs = ((particle.p.x * 256.0 * 256.0) as u16).to_be_bytes();
                    let ys = ((particle.p.y * 256.0 * 256.0) as u16).to_be_bytes();
                    // let cs = (particle.collisions.min(255) as u8).to_be_bytes();
                    let mut status: u8 = 0;
                    if particle.collisions > 0 {
                        status += 1;
                    }
                    if particle.activation > 0.01 {
                        status += 2;
                    }
                    data[i..(2 + i)].copy_from_slice(&xs[..2]);
                    data[(2 + i)..(2 + 2 + i)].copy_from_slice(&ys[..2]);
                    data[(4 + i)..(4 + 1 + i)].copy_from_slice(&status.to_be_bytes()[..1]);
                }
                assert!(data.len() == capacity);
                let m = Message::Binary(data);
                for x in &mut peers.lock().unwrap().values_mut() {
                    match x.user_id {
                        Some(_user_id) => {
                            let mut data = data_common.clone();
                            let mut count: u32 = 0;
                            let p1 = &particles[0];
                            let grid_xy = grid_xy(&p1.p, grid.side);
                            let gx = grid_xy.x as i32;
                            let gy = grid_xy.y as i32;
                            let uu = 64;
                            data.extend(p1.p.x.to_be_bytes());
                            data.extend(p1.p.y.to_be_bytes());
                            let mut status: u8 = 0;
                            if p1.collisions > 0 {
                                status += 1;
                            }
                            if p1.activation > 0.01 {
                                status += 2;
                            }
                            data.extend(status.to_be_bytes());
                            count += 1;
                            for x in gx - uu..gx + uu + 1 {
                                let _x_ = (x as usize + grid.side) % grid.side;
                                for y in gy - uu..gy + uu + 1 {
                                    let _y_ = (y as usize + grid.side) % grid.side;
                                    let gid = grid_id(x as usize, y as usize, grid.side);
                                    for pid2 in &grid.pids[gid] {
                                        let p2 = &particles[*pid2];
                                        data.extend(p2.p.x.to_be_bytes());
                                        data.extend(p2.p.y.to_be_bytes());
                                        let mut status: u8 = 0;
                                        if p2.collisions > 0 {
                                            status += 1;
                                        }
                                        if p2.activation > 0.01 {
                                            status += 2;
                                        }
                                        data.extend(status.to_be_bytes());
                                        count += 1;
                                    }
                                }
                            }
                            data[8 + 7 * 4..8 + 8 * 4].copy_from_slice(&count.to_be_bytes());
                            data[8 + 8 * 4..8 + 9 * 4].copy_from_slice(&1.0_f32.to_be_bytes());
                            let m = Message::Binary(data);
                            if x.tx.start_send(m).is_ok() {
                                // println!("send ok");
                            }
                        }
                        None => {
                            if x.tx.start_send(m.clone()).is_ok() {
                                // println!("send ok");
                            }
                        }
                    }
                }
                *w += 1;
            }
            wait(&syncers[3], world.thread_count);
            elapsed_total += start.elapsed().as_micros();
            step += 1;
            let delta = Duration::from_millis(10);
            if start.elapsed() < delta {
                let sleep_duration = delta - start.elapsed();
                thread::sleep(sleep_duration);
            }
        });
    }
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(
            peers.clone(),
            stream,
            addr,
            users.clone(),
        ));
    }
    Ok(())
}
