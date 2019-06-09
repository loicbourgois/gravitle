use crate::point::Point;
use crate::segment::Segment;
use crate::vector::Vector;

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
    initial_cycle_delta_y: i32,
    thrust_force: f64,
    thrust_activated: bool,
    coordinates_cycled: [f64; 8]
}

//
// Implementation
//
impl Link {

    //
    // Constructor
    //
    pub fn new(p1_index: usize, p2_index: usize, length: f64, strengh: f64,
            thrust_force: f64,
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
            initial_cycle_delta_y,
            thrust_force,
            thrust_activated: false,
            coordinates_cycled: [0.0; 8]
        }
    }

    //
    // Load link parameters from JSON
    //
    pub fn load_from_json(&mut self, json_string: String) {
        let json_parsed = &json::parse(&json_string).unwrap();
        self.length = json_parsed["length"].as_f64().unwrap_or(self.length);
        self.strengh = json_parsed["strengh"].as_f64().unwrap_or(self.strengh);
        self.thrust_force = json_parsed["thrust_force"].as_f64().unwrap_or(self.strengh);
     }

    //
    // Get the thrust vector, following the normal of the link
    //
    pub fn get_thrust_forces(&self, coordinates_cycled: [f64;8]) -> Option<Vector> {
        let dx = coordinates_cycled[2] - coordinates_cycled[0];
        let dy = coordinates_cycled[3] - coordinates_cycled[1];
        let normal_1 = (-dy, dx);
        let normal_2 = (dy, -dx);
        let normalized_normal_option = Point::get_normalized_vector(
            0.0, 0.0, normal_1.0, normal_1.1
        );
        match normalized_normal_option {
            Some(normalized_normal) => {
                return Some(Vector {
                    x: normalized_normal.0 * self.thrust_force,
                    y: normalized_normal.1 * self.thrust_force
                });
            },
            None => {
                return None;
            }
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
    pub fn get_coordinates(& self) -> [f64; 4] {
        [self.x1, self.y1, self.x2, self.y2]
    }

    //
    // Returns x1, y1, x2, y2 as a segment
    //
    pub fn get_coordinates_as_segment(& self) -> Segment {
        Segment {
            x1: self.x1,
            y1: self.y1,
            x2: self.x2,
            y2: self.y2
        }
    }

    //
    // Get cycled coordinates
    // Useful for drawing if wrap around is enabled
    //
    pub fn get_coordinates_cycled(&self) -> [f64; 8] {
        self.coordinates_cycled
    }

    //
    // Returns cycled coordinates as 2 segments
    // Useful to handle cases where a links wrap around the Universe.
    //
    pub fn get_cycled_coordinates_as_segments(&self) -> [Segment; 2] {
        [Segment {
            x1: self.coordinates_cycled[0],
            y1: self.coordinates_cycled[1],
            x2: self.coordinates_cycled[2],
            y2: self.coordinates_cycled[3]
        }, Segment {
            x1: self.coordinates_cycled[4],
            y1: self.coordinates_cycled[5],
            x2: self.coordinates_cycled[6],
            y2: self.coordinates_cycled[7]
        }]
    }

    //
    // Setter for coordinates_cycled
    //
    pub fn set_coordinates_cycled(&mut self, coordinates_cycled: [f64; 8] ) {
        self.coordinates_cycled = coordinates_cycled;
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

    //
    // Activate_thrust
    //
    pub fn activate_thrust(&mut self) {
        self.thrust_activated = true;
    }

    //
    // Deactivate thrust
    //
    pub fn deactivate_thrust(&mut self) {
        self.thrust_activated = false;
    }

    //
    // Setter for thrust_force
    //
    pub fn set_thrust_force(&mut self, thrust_force: f64) {
        self.thrust_force = thrust_force;
    }

    //
    // Getter for thrust_activated
    //
    pub fn is_thrust_activated(&self) -> bool {
        self.thrust_activated
    }
}
