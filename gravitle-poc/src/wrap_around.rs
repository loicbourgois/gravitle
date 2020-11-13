use wasm_bindgen::prelude::*;

//
// A wrap around happens when a particle cross an edge of the Universe.
//
#[wasm_bindgen]
pub struct WrapAround {
    particle_index: usize
}

//
// Implement WrapAround
//
#[wasm_bindgen]
impl WrapAround {

    //
    // Create a new WrapAround
    //
    pub fn new (
            particle_index: usize
    ) -> WrapAround {
        WrapAround {
            particle_index
        }
    }

    //
    // Getter for particle_index
    //
    pub fn get_particle_index(& self) -> usize {
        self.particle_index
    }
}
