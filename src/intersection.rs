use wasm_bindgen::prelude::*;

//
// Intersection point between a moving particle and a static link
//
#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Intersection {
    intersection_x: f64,
    intersection_y: f64,
    link_id: usize,
    particle_id: usize
}

//
//
//
#[wasm_bindgen]
impl Intersection {

    //
    // Constructor
    //
    pub fn new (intersection_x: f64,
            intersection_y: f64,
            link_id: usize,
            particle_id: usize
    ) -> Intersection {
        Intersection {
            intersection_x,
            intersection_y,
            link_id,
            particle_id
        }
    }

    //
    // Getters
    //
    pub fn get_intersection_x(& self) -> f64 {
        self.intersection_x
    }
    pub fn get_intersection_y(& self) -> f64 {
        self.intersection_y
    }
    pub fn get_link_id(& self) -> usize {
        self.link_id
    }
    pub fn get_particle_id(& self) -> usize {
        self.particle_id
    }
}
