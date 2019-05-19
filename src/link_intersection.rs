use wasm_bindgen::prelude::*;

//
// Intersection point between two links
//
#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct LinkIntersection {
    intersection_x: f64,
    intersection_y: f64,
    link_1_index: usize,
    link_2_index: usize
}

//
// Implement LinkIntersection
//
#[wasm_bindgen]
impl LinkIntersection {

    //
    // Constructor
    //
    pub fn new (intersection_x: f64,
            intersection_y: f64,
            link_1_index: usize,
            link_2_index: usize
    ) -> LinkIntersection {
        LinkIntersection {
            intersection_x,
            intersection_y,
            link_1_index,
            link_2_index
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
    pub fn get_link_1_index(& self) -> usize {
        self.link_1_index
    }
    pub fn get_link_2_index(& self) -> usize {
        self.link_2_index
    }
}
