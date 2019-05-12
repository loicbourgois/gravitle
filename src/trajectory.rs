use wasm_bindgen::prelude::*;

//
// Position in a trajectory
//
#[derive(Clone)]
struct Position {
    x: f64,
    y: f64,
    step: u32
}

//
// Trajectory taken by a particle
//
#[wasm_bindgen]
#[derive(Clone)]
pub struct Trajectory {
    particle_id: u32,
    positions: Vec<Position>
}

impl Trajectory {

    //
    // Create a new Trajectory
    //
    pub fn new(particle_id: u32) -> Trajectory {
        Trajectory {
            particle_id: particle_id,
            positions: Vec::new()
        }
    }

    //
    // Add a position to the Trajectory
    //
    pub fn add_position(&mut self, x: f64, y: f64, step: u32) {
        self.positions.push(Position {
            x: x,
            y: y,
            step: step
        });
    }

    //
    // Get positions according to a period
    //
    pub fn get_positions_at_period_as_f64s(& self, period: usize) -> Vec<f64> {
        let mut f64s = Vec::new();
        for i in 0..self.positions.len() {
            if i%period == 0 {
                f64s.push(self.positions[i].x);
                f64s.push(self.positions[i].y);
            } else {
                // Do nothing
            }
        }
        return f64s;
    }
}
