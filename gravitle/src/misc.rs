use crate::grid_id_position;
use crate::Grid;
use crate::SocketAddr;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
pub struct User {
    pub user_id: u128,
    pub addr: SocketAddr,
    pub orders: HashMap<usize, f32>,
    pub ship_pid: usize,
}
pub type Pid = usize;
pub type Links = Vec<[usize; 2]>;
// pub type Users = Arc<Mutex<HashMap<u128, User>>>;
pub struct Configuration {
    pub particle_count: usize,
    pub thread_count: usize,
    pub diameter: f32,
    pub ships_count: usize,
}
#[derive(Clone, Copy, Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}
#[derive(Debug)]
pub struct Delta {
    pub collisions: u32,
    pub p: Vector,
    pub v: Vector,
    pub pid: usize, // particle id
    pub tid: usize, // thread id
    pub dtid: usize,
    pub did: usize,
    pub direction: Vector,
}
pub struct World {
    pub particle_count: usize,
    pub thread_count: usize,
    pub diameter: f32,
    pub particle_per_thread: usize,
    pub particle_diameter_sqrd: f32,
    pub ships_count: usize,
}
impl World {
    pub fn new(c: &Configuration) -> World {
        World {
            particle_count: c.particle_count,
            thread_count: c.thread_count,
            diameter: c.diameter,
            particle_per_thread: c.particle_count / c.thread_count,
            particle_diameter_sqrd: c.diameter * c.diameter,
            ships_count: c.ships_count,
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
