//
// Link between two particles
//
#[derive(Copy, Clone)]
pub struct Link {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    p1_index: usize,
    p2_index: usize
}

//
// Implementation
//
impl Link {

    //
    // Constructor
    //
    pub fn new(p1_index: usize, p2_index: usize) -> Link {
        Link {
            p1_index,
            p2_index,
            x1: 0.0,
            y1: 0.0,
            x2: 0.0,
            y2: 0.0
        }
    }

    //
    // Decrease the index of particle p1
    // Used when deleting a particle which index is inferior to p1 index
    //
    pub fn decrease_p1_index(&mut self) {
        self.p1_index -= 1;
    }

    //
    // Decrease the index of particle p2
    // Used when deleting a particle which index is inferior to p2 index
    //
    pub fn decrease_p2_index(&mut self) {
        self.p2_index -= 1;
    }

    //
    // Check whether the link depend on the given particle
    //
    pub fn has_particle(&self, particle_index: usize) -> bool {
        self.p1_index == particle_index || self.p2_index == particle_index
    }

    // Setter for x1, y1, x2, y2
    //
    pub fn set_coordinates(
            &mut self,
            x1: f64, y1: f64,
            x2: f64, y2: f64
    ) {
        self.x1 = x1;
        self.y1 = y1;
        self.x2 = x2;
        self.y2 = y2;
    }

    //
    // Getter for x1, y1, x2, y2
    //
    pub fn get_coordinates(& self) -> (f64, f64, f64, f64) {
        (self.x1, self.y1, self.x2, self.y2)
    }

    //
    // Getter for p1_index
    //
    pub fn get_p1_index(&self) -> usize {
        self.p1_index
    }

    //
    // Getter for p2_index
    //
    pub fn get_p2_index(&self) -> usize {
        self.p2_index
    }
}
