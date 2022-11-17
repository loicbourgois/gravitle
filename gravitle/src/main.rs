use crate::grid::grid_id_position;
use crate::grid::Grid;
use crate::grid::GridConfiguration;
use crate::math::collision_response;
use crate::math::normalize;
use crate::math::normalize_2;
use crate::math::rotate;
use crate::math::wrap_around;
// use crate::network::FreeShipPids;
use crate::setup::setup_5::reset_ship_particles;
use crate::setup::setup_5::setup_5;
use chrono::Utc;
use crate::particle::Particles;
use particle::Pkind;
mod compute_main;
use crate::compute_main::compute_main;
use rand::Rng;
use std::collections::HashSet;
use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;
use std::time::Instant;
mod grid;
mod math;
mod network;
mod particle;
mod setup;
mod test_math;
mod misc;
use crate::grid::grid_id;
use crate::grid::grid_xy;
use crate::network::handle_connection;
// use crate::network::Peers;
use crate::particle::Particle;
use std::{collections::HashMap, env, io::Error as IoError, net::SocketAddr, sync::Mutex};
use tokio::net::TcpListener;
use tungstenite::protocol::Message;
use uuid::Uuid;
use misc::*;
use crate::network::NetworkData;
use crate::network::SharedNetworkData;
pub type Syncers = Vec<Vec<Arc<RwLock<usize>>>>;
#[tokio::main]
async fn main() -> Result<(), IoError> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "0.0.0.0:8000".to_string());
    // let peers = Peers::new(Mutex::new(HashMap::new()));
    // let users = Users::new(Mutex::new(HashMap::new()));
    // let free_ship_pids = FreeShipPids::new(Mutex::new(HashSet::new()));
    let shared_network_data = SharedNetworkData::new(Mutex::new(NetworkData{
        peers: HashMap::new(),
        users: HashMap::new(),
        free_ship_pids: HashSet::new(),
    }));
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on {}", addr);
    println!("Try at http://localhost/play/");
    let world = World::new(&Configuration {
        particle_count: 50_000,
        thread_count: 5,
        diameter: 0.001,
        ships_count: 4,
    });
    let crd = 0.01; // collision response delta (position)
    let crdv = 0.9; // collision response delta (velocity)
    let link_strengh = 0.1;
    let linkt_length_ratio = 1.01;
    let booster_acceleration = world.diameter * 0.01;
    let mut grid = Grid::new(&GridConfiguration { side: 1024 / 2 });
    assert!(1.0 / grid.side as f32 > world.diameter);
    let mut links: Links = Vec::new();
    let setup = 5;
    let mut particles = Particle::new_particles_4(&world);
    if setup == 5 {
        setup_5(
            &mut links,
            &mut particles,
            &world,
            &mut shared_network_data.lock().unwrap().free_ship_pids,
        );
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
    let mut syncers: Syncers = Vec::new();
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
            let mut rng = rand::thread_rng();
            unsafe {
                let particles = &mut (*particles_ptr.load(Ordering::Relaxed));
                let _particles2 = &mut (*particles_ptr.load(Ordering::Relaxed));
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
                            let p1 = &particles[pid1];
                            for ns in neighbours(&p1.p, grid) {
                                for pid2 in ns {
                                    let p2 = &(*particles2)[*pid2];
                                    if p1.pid < p2.pid {
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
                                        }
                                    }
                                }
                            }
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
                            let factor = (world.diameter * linkt_length_ratio - d) * link_strengh;
                            if wa.d_sqrd < world.particle_diameter_sqrd {
                                let cr = collision_response(&wa, p1, p2);
                                if !cr.x.is_nan() && !cr.y.is_nan() {
                                    {
                                        let d1 = &mut deltas[tid * world.particle_count + p1.pid];
                                        d1.collisions += 1;
                                        d1.v.x -= cr.x * crdv * 0.5;
                                        d1.v.y -= cr.y * crdv * 0.5;
                                        d1.p.x += wa.d.x * crd;
                                        d1.p.y += wa.d.y * crd;
                                    }
                                    {
                                        let d2 = &mut deltas[tid * world.particle_count + p2.pid];
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
                            let mut gun_ok = true;
                            if p1.direction.x.is_nan() || p1.direction.y.is_nan() {
                                p1.direction.x = 0.0;
                                p1.direction.y = 0.0;
                                gun_ok = false;
                            }
                            if let Pkind::Booster = p1.kind {
                                p1.v.x += p1.direction.x * p1.activation * booster_acceleration;
                                p1.v.y += p1.direction.y * p1.activation * booster_acceleration;
                            }
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
                            match p1.kind {
                                Pkind::Gun => {
                                    if gun_ok && p1.activation >= 0.9 {
                                        let mut p2 = &mut particles2
                                            [rng.gen_range(p1.pid + 20..p1.pid + 500)];
                                        p2.p.x = p1.p.x - p1.direction.x * world.diameter * 1.1;
                                        p2.p.y = p1.p.y - p1.direction.y * world.diameter * 1.1;
                                        p2.pp.x =
                                            p2.p.x + p1.direction.x * world.diameter * 0.5 - p1.v.x;
                                        p2.pp.y =
                                            p2.p.y + p1.direction.y * world.diameter * 0.5 - p1.v.y;
                                        for tid in 0..world.thread_count {
                                            let d2 =
                                                &mut deltas[tid * world.particle_count + p2.pid];
                                            d2.collisions = 0;
                                            d2.p.x = 0.0;
                                            d2.p.y = 0.0;
                                            d2.v.x = 0.0;
                                            d2.v.y = 0.0;
                                            d2.direction.x = 0.0;
                                            d2.direction.y = 0.0;
                                        }
                                        p1.activation = -1.0;
                                    }
                                }
                                _ => {}
                            }
                        }
                        *w += 1;
                    }
                    wait(&syncers[2], tid);
                    //
                    //
                    //
                    //
                    {
                        let mut w = syncers[3][tid].write().unwrap();
                        *w += 1;
                    }
                    wait(&syncers[3], tid);
                }
            }
        }));
    }
    let shared_network_data_2 = shared_network_data.clone();
    // let aa: & 'static mut Particles mut = &mut particles;
    // move || compute_main(
    //     shared_network_data.clone(),
    //     syncers,
    //     world,
    //     aa,
    //     grid,
    // );

    {
        //     // let peers = peers.clone();
        //     // let users = users.clone();
        //     // let free_ship_pids = free_ship_pids.clone();
    
        //     let shared_network_data = shared_network_data.clone();
        let mut elapsed_total = 0;
        let mut step = 0;
            let mut elapsed_network = Instant::now().elapsed().as_micros();
            thread::spawn(move || loop {
                let start = Instant::now();
                // 
                // Step 0
                // 
                {
                    let mut w = syncers[0][world.thread_count].write().unwrap();
                    grid.update_01();
                    grid.update_02(&mut particles);
                    for user in shared_network_data.lock().unwrap().users.values_mut() {
                        for (pid, activation) in &user.orders {
                            let mut p1 = &mut particles[*pid + user.ship_pid];
                            match p1.kind {
                                Pkind::Booster => {
                                    p1.activation = *activation;
                                }
                                Pkind::Core => {
                                    p1.activation = *activation;
                                }
                                Pkind::Gun => {
                                    if approx_equal(*activation, 0.0)
                                        || approx_equal(*activation, 1.0)
                                            && approx_equal(p1.activation, 0.0)
                                    {
                                        p1.activation = *activation;
                                    }
                                }
                                _ => {}
                            }
                        }
                        user.orders.clear();
                    }
                    *w += 1;
                }
                wait(&syncers[0], world.thread_count);
                //
                // Step 1
                //
                {
                    let mut w = syncers[1][world.thread_count].write().unwrap();
                    *w += 1;
                }
                wait(&syncers[1], world.thread_count);
                // 
                // Step 2
                // 
                {
                    let mut w = syncers[2][world.thread_count].write().unwrap();
                    *w += 1;
                }
                wait(&syncers[2], world.thread_count);
                //
                // Step 3
                //
                {
                    let mut w = syncers[3][world.thread_count].write().unwrap();
                    let mut collisions_count = 0;
                    let mut ships_to_reset = HashSet::new();
                    for p1 in particles.iter() {
                        collisions_count += p1.collisions;
                        if p1.kind == Pkind::Core && p1.activation >= 0.9 {
                            ships_to_reset.insert(p1.pid);
                        }
                    }
                    for pid in ships_to_reset {
                        reset_ship_particles(pid, &mut particles, &world);
                    }
                    let elapsed_compute = start.elapsed().as_micros();
                    let start_network = Instant::now();
                    let part_bytes = 2 + 2 + 1 + 1;
                    let common_capacity = 4 * 12 + 8;
                    let capacity = world.particle_count * part_bytes + common_capacity;
                    let mut data = vec![0; capacity];
                    let mut data_common = Vec::with_capacity(common_capacity);
                    data_common.extend(Utc::now().timestamp_millis().to_be_bytes().to_vec());
                    data_common.extend((step as f32).to_be_bytes().to_vec());
                    data_common.extend((elapsed_total as f32).to_be_bytes().to_vec());
                    data_common.extend((elapsed_compute as f32).to_be_bytes().to_vec());
                    data_common.extend((elapsed_total as f32).to_be_bytes().to_vec());
                    data_common.extend((shared_network_data.lock().unwrap().peers.len() as u32).to_be_bytes().to_vec());
                    data_common.extend(collisions_count.to_be_bytes().to_vec());
                    data_common.extend((world.diameter).to_be_bytes().to_vec());
                    data_common.extend((world.particle_count as u32).to_be_bytes().to_vec());
                    data_common.extend(((256.0 * 256.0) as f32).to_be_bytes().to_vec());
                    data_common.extend((elapsed_network as f32).to_be_bytes().to_vec());
                    data_common.extend((world.ships_count as u32).to_be_bytes().to_vec());
                    data_common.extend(
                        (shared_network_data.lock().unwrap().free_ship_pids.len() as u32)
                            .to_be_bytes()
                            .to_vec(),
                    );
                    data[..common_capacity].copy_from_slice(&data_common);
                    let _data_2: Vec<u8> = vec![0; part_bytes * world.particle_count];
                    for (pid, particle) in particles.iter().enumerate() {
                        let i = common_capacity + pid * part_bytes;
                        let xs = ((particle.p.x * 256.0 * 256.0) as u16).to_be_bytes();
                        let ys = ((particle.p.y * 256.0 * 256.0) as u16).to_be_bytes();
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
                        data[(5 + i)..(5 + 1 + i)]
                            .copy_from_slice(&(particle.kind as u8).to_be_bytes()[..1]);
                    }
                    assert!(data.len() == capacity);
                    let m = Message::Binary(data);

                    {
                    let mut network_data = shared_network_data.lock().unwrap();
                    let mut ship_pids = HashMap::new();
                    for (k,v) in &network_data.users {
                        ship_pids.insert(*k, v.ship_pid);
                    }
                    for peer in &mut network_data.peers.values_mut() {
                        match peer.user_id {
                            Some(user_id) => {
                                let ship_pid = ship_pids.get(&user_id).unwrap();
                                let mut data = data_common.clone();
                                let mut count: u32 = 0;
                                let p1 = &particles[*ship_pid];
                                let grid_xy = grid_xy(&p1.p, grid.side);
                                let gx = grid_xy.x as i32;
                                let gy = grid_xy.y as i32;
                                let uu = 32;
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
                                data.extend((p1.kind as u8).to_be_bytes());
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
                                            data.extend((p2.kind as u8).to_be_bytes());
                                            count += 1;
                                        }
                                    }
                                }
                                data[8 + 7 * 4..8 + 8 * 4].copy_from_slice(&count.to_be_bytes());
                                data[8 + 8 * 4..8 + 9 * 4].copy_from_slice(&1.0_f32.to_be_bytes());
                                let m = Message::Binary(data);
                                if peer.tx.start_send(m).is_ok() {
                                    // println!("send ok");
                                }
                            }
                            None => {
                                if peer.tx.start_send(m.clone()).is_ok() {
                                    // println!("send ok");
                                }
                            }
                        }
                    }
                }
                    elapsed_network = start_network.elapsed().as_micros();
                    *w += 1;
                }
                wait(&syncers[3], world.thread_count);
                //
                // More
                //
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
            stream,
            addr,
            shared_network_data_2.clone(),
        ));
    }
    Ok(())
}

fn approx_equal(a: f32, b: f32) -> bool {
    (a - b).abs() < 0.000001
}
