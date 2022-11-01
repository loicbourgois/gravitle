use std::thread;
use std::time::Duration;
use std::time::Instant;
use std::sync::atomic::AtomicPtr;
use std::sync::RwLock;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use rand::Rng;
use crate::grid::GridConfiguration;
use crate::grid::Grid;
use crate::math::wrap_around;
use crate::grid::grid_id_position;
mod grid;
mod math;
struct Configuration {
    particle_count: usize,
    thread_count: usize,
    diameter: f32,
}
#[derive(Clone, Copy)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}
type Particles = Vec<Particle>;
pub struct Particle {
    pub p: Vector,
    pub v: Vector,
    pub pp: Vector,
    pub m: f32,
    pub collisions: usize,
    pub pid: usize, // particle id
    pub tid: usize, // thread id
    pub gid: usize,
}
struct Delta {
    collisions: usize,
    pid: usize, // particle id
    tid: usize, // thread id
    dtid: usize,
    did: usize,
}
struct World {
    particle_count: usize,
    thread_count: usize,
    diameter: f32,
    particle_per_thread: usize,
    particle_diameter_sqrd: f32,
}
impl World {
    pub fn new(c: &Configuration) -> World {
        World {
            particle_count: c.particle_count,
            thread_count: c.thread_count,
            diameter: c.diameter,
            particle_per_thread: c.particle_count/c.thread_count,
            particle_diameter_sqrd: c.diameter * c.diameter,
        }
    }
}
pub fn neighbours<'a>(position: &'a Vector, grid: &'a Grid) -> [&'a Vec<usize>; 9] {
    let gid = grid_id_position(position, grid.side);
    return [
        &grid.pids[grid.gids[gid][0]],
        &grid.pids[grid.gids[gid][1]],
        &grid.pids[grid.gids[gid][2]],
        &grid.pids[grid.gids[gid][3]],
        &grid.pids[grid.gids[gid][4]],
        &grid.pids[grid.gids[gid][5]],
        &grid.pids[grid.gids[gid][6]],
        &grid.pids[grid.gids[gid][7]],
        &grid.pids[grid.gids[gid][8]],
    ];
}
pub fn wait(subsyncers: &Vec<Arc<RwLock<usize>>>, i: usize) {
    loop {
        let mut ok = true;
        for s in subsyncers {
            let a = *(subsyncers[i].read().unwrap());
            let b = *(s.read().unwrap());
            if a > b || a < b-1 {
                ok = false;
                break;
            }
        }
        if ok {
            break;
        }
    }
}
fn main() {
    let world = World::new(&Configuration {
        particle_count: 100_000,
        thread_count: 5,
        diameter: 0.001,
    });
    let mut grid = Grid::new(&GridConfiguration {
        side: 1000,
    });
    let mut particles = Vec::new();
    let mut deltas = Vec::new();
    let mut rng = rand::thread_rng();
    for dtid in 0..world.thread_count {
        for pid in 0..world.particle_count {
            let tid = pid % world.thread_count;
            assert!( deltas.len() == dtid * world.particle_count + pid );
            deltas.push(Delta {
                collisions: 0,
                pid: pid,
                tid: tid,
                dtid: dtid,
                did: deltas.len(),
            });
        }
    }
    for pid in 0..world.particle_count {
        let tid = pid % world.thread_count;
        let p = Vector {
            x: rng.gen::<f32>(),
            y: rng.gen::<f32>(),
        };
        let v = Vector {
            x: world.diameter * 0.9 * rng.gen::<f32>() - 0.5 * world.diameter * 0.9,
            y: world.diameter * 0.9 * rng.gen::<f32>() - 0.5 * world.diameter * 0.9,
        };
        particles.push(Particle{
            p: p,
            pp: Vector {
                x: p.x - v.x,
                y: p.y - v.y,
            },
            v: v,
            m: rng.gen(),
            tid: tid,
            pid: pid,
            collisions: 0,
            gid: 0,
        });
    }
    for tid in 0..world.thread_count {
        for i in 0..world.particle_per_thread {
            let pid = i*world.thread_count + tid;
            assert!( particles[pid].pid == pid );
            assert!( particles[pid].tid == tid );
        }
    }
    assert!( deltas.len() == world.particle_count * world.thread_count );
    for did in 0..deltas.len() {
        let delta = &deltas[did];
        assert!( delta.did == delta.dtid * world.particle_count + delta.pid );
        assert!( delta.pid == did % world.particle_count );
        assert!( delta.tid == (did % world.particle_count) % world.thread_count );
        assert!( delta.dtid == did / world.particle_count );
        assert!( delta.did == did );
        assert!( particles[delta.pid].pid == delta.pid );
        assert!( particles[delta.pid].tid == delta.tid );
    }
    let mut threads = Vec::new();
    let mut syncers = Vec::new();
    for _ in 0..4 {
        let mut subsyncers = Vec::new();
        for _ in 0..world.thread_count+1 {
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
                let grid = & (*grid_ptr.load(Ordering::Relaxed));
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
                            let pid1 = i*world.thread_count + tid;
                            let mut p1 = &mut particles[pid1];
                            p1.collisions = 0;
                            for ns in neighbours(&p1.p, &grid) {
                                for pid2 in ns {
                                    let p2 = &mut (*particles2)[*pid2];
                                    if p1.pid < p2.pid {
                                        let wa = wrap_around(&p1.p, &p2.p);
                                        if wa.d_sqrd < world.particle_diameter_sqrd {
                                            let did1 = tid * world.particle_count + p1.pid ;
                                            deltas[did1].collisions += 1;
                                            let did2 = tid * world.particle_count + p2.pid ;
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
                            let pid1 = i*world.thread_count + tid;
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
    let mut c = 0;
    loop {
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
            *w += 1;
        }
        wait(&syncers[3], world.thread_count);
        elapsed_total += start.elapsed().as_micros();
        c += 1;
        println!("{} μs", elapsed_total/c);
        thread::sleep(Duration::from_millis(10));
    }
}
