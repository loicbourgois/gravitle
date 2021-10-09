use wasm_bindgen::prelude::*;

//
// Collision between two particles
//
#[wasm_bindgen]
pub struct Collision {
    particle_1_index: usize,
    particle_2_index: usize
}

//
// Implement Collision
//
#[wasm_bindgen]
impl Collision {

    //
    // Create a new collision between 2 particles
    //
    pub fn new (
            particle_1_index: usize,
            particle_2_index: usize
    ) -> Collision {
        Collision {
            particle_1_index,
            particle_2_index
        }
    }

    //
    // Getters
    //
    pub fn get_particle_1_index(& self) -> usize {
        self.particle_1_index
    }
    pub fn get_particle_2_index(& self) -> usize {
        self.particle_2_index
    }
}
