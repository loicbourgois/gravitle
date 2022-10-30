use crate::wrap_around;
use crate::Particle;
// use crate::neighbours;
use crate::grid_id_particle;
use crate::new_particles;
use crate::Grid;
use crate::Particles;
use std::collections::HashSet;

pub struct World {
    particle_diameter: f32,
    particle_diameter_sqrd: f32,
    grid: Grid,
    particles: Particles,
    pub step: usize,
}

pub fn neighbours<'a>(particle: &'a Particle, grid: &'a Grid) -> [&'a HashSet<usize>; 9] {
    let gid = grid_id_particle(particle, grid.side);
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
    pub fn new(particle_diameter: f32, particle_count: usize, grid_side: usize) -> World {
        assert!((particle_diameter * grid_side as f32) <= 1.0);
        World {
            particle_diameter: particle_diameter,
            grid: Grid::new(grid_side),
            particles: new_particles(particle_count),
            particle_diameter_sqrd: particle_diameter * particle_diameter,
            step: 0,
        }
    }
    pub fn update(&mut self) {
        let mut collissions = 0;
        self.grid.update(&mut self.particles);
        for p1 in &self.particles {
            for pidx2s in neighbours(&p1, &self.grid) {
                for pidx2 in pidx2s {
                    let p2: &Particle = &self.particles[*pidx2];
                    if (p1.idx < p2.idx) {
                        let wa = wrap_around(&p1.p, &p2.p);
                        if wa.d_sqrd < self.particle_diameter_sqrd {
                            collissions += 1;
                        }
                    }
                }
            }
        }
        self.step += 1;
    }
    pub fn update_2(&mut self) {
        // let mut collissions = 0;
        self.grid.update(&mut self.particles);
        for p1 in &self.particles {
            for pidx2s in neighbours(&p1, &self.grid) {
                for pidx2 in pidx2s {
                    let p2: &Particle = &self.particles[*pidx2];
                    if (p1.idx < p2.idx) {
                        let wa = wrap_around(&p1.p, &p2.p);
                        if wa.d_sqrd < self.particle_diameter_sqrd {
                            // collissions += 1;
                        }
                    }
                }
            }
        }
        self.step += 1;
    }
}
