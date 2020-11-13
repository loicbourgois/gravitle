use crate::point::Point;

//
// Represent a vector in 2d space
//
#[derive(Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64
}

impl Vector {

    //
    // Constructor
    //
    pub fn new(p1: & Point, p2: & Point) -> Vector {
        Vector {
            x: p2.x - p1.x,
            y: p2.y - p1.y
        }
    }

    //
    // Multiply the vector by a floating point number
    //
    pub fn muliply(&mut self, multiplier: f64) {
        self.x *= multiplier;
        self.y *= multiplier;
    }

    //
    // Return a normalized version of the vector
    //
    pub fn normalized(& self) -> Vector {
        let length = self.length();
        Vector {
            x: self.x / length,
            y: self.y / length
        }
    }

    //
    // Return a multiplied version of the vector
    //
    pub fn multiplied(& self, multiplier: f64) -> Vector {
        Vector {
            x: self.x * multiplier,
            y: self.y * multiplier
        }
    }

    //
    // Returns the length of the vector
    //
    pub fn length(& self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    //
    // Returns the dot product of two vectors
    //
    pub fn dot(v1: Vector, v2: Vector) -> f64 {
        v1.x * v2.x + v1.y * v2.y
    }

    //
    // Helper function to get a normalized vector
    //
    // Returns None if the length of the initial vector
    // is inferior or equal to 0
    //
    pub fn get_normalized_vector(x1: f64, y1: f64, x2: f64, y2: f64) -> Option<Vector> {
        let length = Point::get_distance(x1, y1, x2, y2);
        let delta_x = x2 - x1;
        let delta_y = y2 - y1;
        if length > 0.0 {
            let x = delta_x / length;
            let y = delta_y / length;
            Some(Vector{x, y})
        } else {
            None
        }
    }
}
