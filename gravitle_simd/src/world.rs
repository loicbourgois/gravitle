use crate::wrap_around;
use crate::Particle;
// use crate::neighbours;
use crate::grid::grid_id_position;
use crate::grid_id_particle;
use crate::new_particles;
use crate::Grid;
use crate::Particles;
use crate::Vector;
use std::collections::HashSet;

pub struct World {
    pub particle_diameter: f32,
    pub particle_diameter_sqrd: f32,
    pub grid: Grid,
    pub particles: Particles,
    pub step: usize,
    pub collissions: usize,
    pub thread_count: usize,
    pub particles_per_thread: usize,
}

pub fn neighbours<'a>(position: &'a Vector, grid: &'a Grid) -> [&'a HashSet<usize>; 9] {
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


pub struct WrapperType(pub *mut World);


unsafe impl Send for WrapperType {}
unsafe impl Sync for WrapperType {}


impl World {
    pub fn new(particle_diameter: f32, particle_count: usize, grid_side: usize, thread_count: usize) -> World {
        assert!((particle_diameter * grid_side as f32) <= 1.0);
        World {
            particle_diameter: particle_diameter,
            grid: Grid::new(grid_side),
            particles: new_particles(particle_count, particle_diameter),
            particle_diameter_sqrd: particle_diameter * particle_diameter,
            step: 0,
            collissions: 0,
            thread_count: thread_count,
            particles_per_thread: particle_count/thread_count,
        }
    }
    pub fn reset_common(& self) {

    }
    pub fn reset(&mut self, thread_id: usize) {
        // let start = thread_id*self.particles_per_thread;
        // let end = start + self.particles_per_thread;
        // for idx in start..end  {
        //     let p1 = &mut self.particles[idx];
        //     p1.colliding = 0;
        // }
    }
    pub fn update(&mut self) {
        self.collissions = 0;
        self.grid.update(&mut self.particles);
        for p1 in &mut self.particles {
            p1.colliding = 0;
        }
        unsafe {
            let particles = &mut self.particles as *mut Particles;
            for p1 in &mut self.particles {
                for pidx2s in neighbours(&p1.p, &self.grid) {
                    for pidx2 in pidx2s {
                        let p2: &mut Particle = &mut (*particles)[*pidx2];
                        if (p1.idx < p2.idx) {
                            let wa = wrap_around(&p1.p, &p2.p);
                            if wa.d_sqrd < self.particle_diameter_sqrd {
                                self.collissions += 1;
                                p1.colliding = 1;
                                p2.colliding = 1;
                            }
                        }
                    }
                }
            }
        }
        for p1 in &mut self.particles {
            p1.v.x = p1.p.x - p1.pp.x;
            p1.v.y = p1.p.y - p1.pp.y;
            p1.p.x = (1.0 + p1.p.x + p1.v.x) % 1.0;
            p1.p.y = (1.0 + p1.p.y + p1.v.y) % 1.0;
            p1.pp.x = p1.p.x - p1.v.x;
            p1.pp.y = p1.p.y - p1.v.y;
        }
        self.step += 1;
    }
    pub fn update_b(&mut self) {
        self.collissions = 0;
        self.grid.update(&mut self.particles);
        for p1 in &self.particles {
            for pidx2s in neighbours(&p1.p, &self.grid) {
                for pidx2 in pidx2s {
                    let p2: &Particle = &self.particles[*pidx2];
                    if (p1.idx < p2.idx) {
                        let wa = wrap_around(&p1.p, &p2.p);
                        if wa.d_sqrd < self.particle_diameter_sqrd {
                            self.collissions += 1;
                        }
                    }
                }
            }
        }
        self.step += 1;
    }
}
