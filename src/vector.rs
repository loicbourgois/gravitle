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
}
