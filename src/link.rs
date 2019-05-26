//
// Link between two particles
//
#[derive(Copy, Clone)]
pub struct Link {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    length: f64,
    strengh: f64,
    p1_index: usize,
    p2_index: usize,
    enabled: bool,
    initial_cycle_delta_x: i32,
    initial_cycle_delta_y: i32
}

//
// Implementation
//
impl Link {

    //
    // Constructor
    //
    pub fn new(p1_index: usize, p2_index: usize, length: f64, strengh: f64,
            initial_cycle_delta_x: i32, initial_cycle_delta_y: i32) -> Link {
        Link {
            p1_index,
            p2_index,
            x1: 0.0,
            y1: 0.0,
            x2: 0.0,
            y2: 0.0,
            length,
            strengh,
            enabled: true,
            initial_cycle_delta_x,
            initial_cycle_delta_y
        }
    }

    //
    // Load link parameters from JSON
    //
    pub fn load_from_json(&mut self, json_string: String) {
        let json_parsed = &json::parse(&json_string).unwrap();
        self.length = json_parsed["length"].as_f64().unwrap_or(self.length);
        self.strengh = json_parsed["strengh"].as_f64().unwrap_or(self.strengh);
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
    pub fn get_coordinates(& self) -> [f64; 4] {
        [self.x1, self.y1, self.x2, self.y2]
    }

    //
    // Get cycled coordinates
    // Useful for drawing if wrap around is enabled
    //
    pub fn get_coordinates_cycled(& self, dx: f64, dy: f64) -> [f64; 8] {
        [
            self.x1, self.y1, self.x2 + dx, self.y2 + dy,
            self.x1 - dx, self.y1 - dy, self.x2, self.y2
        ]
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

    //
    // Getter for length
    //
    pub fn get_length(&self) -> f64 {
        self.length
    }

    //
    // Getter for strengh
    //
    pub fn get_strengh(&self) -> f64 {
        self.strengh
    }

    //
    // Getter for enabled
    //
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    //
    // Getter for initial_cycle_delta_x
    //
    pub fn get_initial_cycle_delta_x(&self) -> i32 {
        self.initial_cycle_delta_x
    }

    //
    // Getter for initial_cycle_delta_y
    //
    pub fn get_initial_cycle_delta_y(&self) -> i32 {
        self.initial_cycle_delta_y
    }
}
