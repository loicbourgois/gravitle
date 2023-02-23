use crate::grid::grid_id_position;
use crate::new_particles;
use crate::particle::new_particle_deltas;
use crate::particle::wrap_around;
use crate::particle::ParticleDeltas;
use crate::particle::TParticleDeltas;
use crate::particle::TParticles;
use crate::Grid;
use crate::Particles;
use crate::Vector;
use std::collections::HashSet;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;
use std::thread;
use std::time::Duration;
use std::time::Instant;

pub struct World {
    pub particle_diameter: f32,
    pub particle_diameter_sqrd: f32,
    pub grid: Arc<RwLock<Grid>>,
    pub particles: Particles,
    pub particle_deltas: ParticleDeltas,
    pub step: usize,
    pub collissions: usize,
    pub thread_count: usize,
    pub particles_per_thread: usize,
    pub particle_count: usize,
}

pub fn neighbours<'a>(position: &'a Vector, grid: &'a Grid) -> [&'a Vec<(usize, usize)>; 9] {
    let gid = grid_id_position(position, grid.side);
    return [
        &grid.pidxs[grid.gids[gid][0]],
        &grid.pidxs[grid.gids[gid][1]],
        &grid.pidxs[grid.gids[gid][2]],
        &grid.pidxs[grid.gids[gid][3]],
        &grid.pidxs[grid.gids[gid][4]],
        &grid.pidxs[grid.gids[gid][5]],
        &grid.pidxs[grid.gids[gid][6]],
        &grid.pidxs[grid.gids[gid][7]],
        &grid.pidxs[grid.gids[gid][8]],
    ];
}

impl World {
    pub fn new(
        particle_diameter: f32,
        particle_count: usize,
        grid_side: usize,
        thread_count: usize,
    ) -> World {
        assert!((particle_diameter * grid_side as f32) <= 1.0);
        let particles_per_thread = particle_count / thread_count;
        World {
            particle_diameter: particle_diameter,
            grid: Arc::new(RwLock::new(Grid::new(grid_side))),
            particles: new_particles(particle_diameter, thread_count, particles_per_thread),
            particle_diameter_sqrd: particle_diameter * particle_diameter,
            step: 0,
            collissions: 0,
            thread_count: thread_count,
            particles_per_thread: particles_per_thread,
            particle_deltas: new_particle_deltas(thread_count, particles_per_thread),
            particle_count: particle_count,
        }
    }

    pub fn update_00(&mut self) {
        self.collissions = 0;
        let mut grid = self.grid.write().unwrap();
        let mut particles_w: Vec<RwLockWriteGuard<TParticles>> =
            self.particles.iter().map(|x| x.write().unwrap()).collect();
        grid.update_01();
        for particles in &mut particles_w {
            grid.update_02(particles);
        }
    }

    pub fn update_01(tparticles: &mut TParticles, tdeltas: &mut TParticleDeltas) {
        for p1 in tparticles {
            p1.collisions = 0;
        }
        for p1 in tdeltas {
            p1.collisions = 0;
        }
    }

    pub fn update_02(
        particles: &Particles,
        tdeltas: &mut TParticleDeltas,
        grid: &Grid,
        particle_diameter_sqrd: f32,
    ) {
        let mut particles_r: Vec<RwLockReadGuard<TParticles>> =
            particles.iter().map(|x| x.read().unwrap()).collect();
        for tparticles in &particles_r {
            for p1 in tparticles.iter() {
                for ns in neighbours(&p1.p, grid) {
                    for n in ns {
                        let thid = n.0;
                        let idx = n.1;
                        let p2 = &particles_r[thid][idx];
                        if (p1.fidx < p2.fidx) {
                            let wa = wrap_around(&p1.p, &p2.p);
                            if wa.d_sqrd < particle_diameter_sqrd {
                                tdeltas[p1.fidx].collisions += 1;
                                tdeltas[p2.fidx].collisions += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn update_03(tparticles: &mut TParticles, deltas: &ParticleDeltas) {
        let deltas_r: Vec<RwLockReadGuard<TParticleDeltas>> =
            deltas.iter().map(|x| x.read().unwrap()).collect();
        for p1 in tparticles {
            for xx in &deltas_r {
                let delta = &xx[p1.fidx];
                p1.collisions += delta.collisions;
            }
            p1.v.x = p1.p.x - p1.pp.x;
            p1.v.y = p1.p.y - p1.pp.y;
            p1.p.x = (1.0 + p1.p.x + p1.v.x) % 1.0;
            p1.p.y = (1.0 + p1.p.y + p1.v.y) % 1.0;
            p1.pp.x = p1.p.x - p1.v.x;
            p1.pp.y = p1.p.y - p1.v.y;
        }
    }

    pub fn update_04(&mut self) {
        self.step += 1;
    }
}
