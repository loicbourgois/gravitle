use crate::grid::grid_id_position;
use crate::grid::Grid;
use crate::grid::GridConfiguration;
use crate::math::collision_response;
use crate::math::normalize;
use crate::math::normalize_2;
use crate::math::rotate;
use crate::math::wrap_around;
use crate::setup::setup_5::reset_ship_particles;
use crate::setup::setup_5::setup_5;
use chrono::Utc;

mod compute_child;
mod compute_main;
use crate::compute_main::compute_main;
use compute_child::compute_child;

use std::collections::HashSet;


use std::sync::Arc;
use std::sync::RwLock;



mod grid;
mod math;
mod misc;
mod network;
mod particle;
mod setup;
mod test_math;
use crate::grid::grid_id;
use crate::grid::grid_xy;
use crate::network::handle_connection;
use crate::network::NetworkData;
use crate::network::SharedNetworkData;
use crate::particle::Particle;
use misc::*;
use std::{collections::HashMap, env, io::Error as IoError, net::SocketAddr, sync::Mutex};
use tokio::net::TcpListener;

use uuid::Uuid;
pub type Deltas = Vec<Delta>;
pub type Syncers = Vec<Vec<Arc<RwLock<usize>>>>;
#[tokio::main]
async fn main() -> Result<(), IoError> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "0.0.0.0:8000".to_string());
    let shared_network_data = SharedNetworkData::new(Mutex::new(NetworkData {
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
    let mut deltas: Deltas = Vec::new();
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
    let mut syncers: Syncers = Vec::new();
    for _ in 0..4 {
        let mut subsyncers = Vec::new();
        for _ in 0..world.thread_count + 1 {
            subsyncers.push(Arc::new(RwLock::new(0)));
        }
        syncers.push(subsyncers)
    }
    compute_child(
        &world,
        booster_acceleration,
        crd,
        crdv,
        link_strengh,
        linkt_length_ratio,
        syncers.clone(),
        &mut grid,
        &mut deltas,
        &mut particles,
        &mut links,
    );
    compute_main(shared_network_data.clone(), syncers, world, particles, grid);
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, addr, shared_network_data.clone()));
    }
    Ok(())
}
