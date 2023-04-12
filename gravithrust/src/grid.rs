use crate::math::Vector;
use crate::particle::Particle;
type Particles = Vec<Particle>;
pub struct Grid {
    pub pidxs: Vec<Vec<usize>>, // particle indexes
    pub gids: Vec<Vec<usize>>,  // grid ids
    pub side: usize,
}

impl Grid {
    pub fn new(side: usize) -> Grid {
        let mut grid = Grid {
            side,
            pidxs: Vec::new(),
            gids: Vec::new(),
        };
        for y in 0..side {
            for x in 0..side {
                let grid_xs = [
                    (x - 1 + side) % side,
                    (x + side) % side,
                    (x + 1 + side) % side,
                ];
                let grid_ys = [
                    (y - 1 + side) % side,
                    (y + side) % side,
                    (y + 1 + side) % side,
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
                grid.pidxs.push(Vec::new());
            }
        }
        grid
    }

    pub fn update_01(&mut self) {
        for x in &mut self.pidxs {
            x.clear();
        }
    }

    pub fn update_02(&mut self, particles: &mut Particles) {
        for p in particles {
            let grid_id_ = grid_id_particle(p, self.side);
            self.pidxs[grid_id_].push(p.idx);
            p.grid_id = grid_id_;
        }
    }
}

pub fn grid_id(x: usize, y: usize, side: usize) -> usize {
    (y % side) * side + x % side
}

pub fn grid_id_particle(particle: &Particle, side: usize) -> usize {
    let side_f32: f32 = side as f32;
    let x: usize = (particle.p.x * side_f32) as usize;
    let y: usize = (particle.p.y * side_f32) as usize;
    grid_id(x, y, side)
}

pub fn grid_id_position(position: Vector, side: usize) -> usize {
    let side_f32: f32 = side as f32;
    let x: usize = (position.x * side_f32) as usize;
    let y: usize = (position.y * side_f32) as usize;
    grid_id(x, y, side)
}
