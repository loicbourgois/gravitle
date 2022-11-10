use crate::particle::Particle;
use crate::particle::Particles;
use crate::Vector;
use crate::Vector_u;

pub struct GridConfiguration {
    pub side: usize,
}

pub struct Grid {
    pub pids: Vec<Vec<usize>>,
    pub gids: Vec<Vec<usize>>,
    pub cell_count: usize,
    pub side: usize,
}

impl Grid {
    pub fn new(configuration: &GridConfiguration) -> Grid {
        let mut grid = Grid {
            side: configuration.side,
            cell_count: configuration.side * configuration.side,
            pids: Vec::new(),
            gids: Vec::new(),
        };
        for y in 0..grid.side {
            for x in 0..grid.side {
                let grid_xs = [
                    (x - 1 + grid.side) % grid.side,
                    (x + grid.side) % grid.side,
                    (x + 1 + grid.side) % grid.side,
                ];
                let grid_ys = [
                    (y - 1 + grid.side) % grid.side,
                    (y + grid.side) % grid.side,
                    (y + 1 + grid.side) % grid.side,
                ];
                assert!(grid_id(x, y, grid.side) == grid.gids.len());
                grid.gids.push(vec![
                    grid_id(grid_xs[0], grid_ys[0], grid.side),
                    grid_id(grid_xs[0], grid_ys[1], grid.side),
                    grid_id(grid_xs[0], grid_ys[2], grid.side),
                    grid_id(grid_xs[1], grid_ys[0], grid.side),
                    grid_id(grid_xs[1], grid_ys[1], grid.side),
                    grid_id(grid_xs[1], grid_ys[2], grid.side),
                    grid_id(grid_xs[2], grid_ys[0], grid.side),
                    grid_id(grid_xs[2], grid_ys[1], grid.side),
                    grid_id(grid_xs[2], grid_ys[2], grid.side),
                ]);
                grid.pids.push(Vec::new())
            }
        }
        grid
    }

    pub fn update_01(&mut self) {
        for x in self.pids.iter_mut() {
            x.clear()
        }
    }

    pub fn update_02(&mut self, particles: &mut Particles) {
        for p in particles {
            let gid = grid_id_particle(p, self.side);
            self.pids[gid].push(p.pid);
            p.gid = gid
        }
    }
}

pub fn grid_id(x: usize, y: usize, side: usize) -> usize {
    (y % side) * side + x % side
}

pub fn grid_id_particle(particle: &Particle, side: usize) -> usize {
    grid_id_position(&particle.p, side)
}

pub fn grid_id_position(position: &Vector, side: usize) -> usize {
    let side_f32: f32 = side as f32;
    let x: usize = (position.x * side_f32) as usize;
    let y: usize = (position.y * side_f32) as usize;
    grid_id(x, y, side)
}

pub fn grid_xy(position: &Vector, side: usize) -> Vector_u {
    let side_f32: f32 = side as f32;
    let x: usize = (position.x * side_f32) as usize;
    let y: usize = (position.y * side_f32) as usize;
    Vector_u { x, y }
}
