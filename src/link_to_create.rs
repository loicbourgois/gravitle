use wasm_bindgen::prelude::*;

//
// Link to be created between two particles
//
#[wasm_bindgen]
#[derive(Clone)]
pub struct LinkToCreate {
    particle_1_index: usize,
    particle_2_index: usize
}

//
// Implementation
//
#[wasm_bindgen]
impl LinkToCreate {

    //
    // Create a new Link to be created between two particles
    // Doesn't actually create the link
    //
    pub fn new (
            particle_1_index: usize,
            particle_2_index: usize
    ) -> LinkToCreate {
        LinkToCreate {
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
