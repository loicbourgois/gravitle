#![allow(warnings)]
use crate::point::Point;
use serde::{Deserialize, Serialize};

//
// Represent a vector in 2d space
//
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector {
    //
    // Constructor
    //
    pub fn new(p1: &Point, p2: &Point) -> Vector {
        Vector {
            x: p2.x - p1.x,
            y: p2.y - p1.y,
        }
    }
    pub fn new_2(x1: f32, y1: f32, x2: f32, y2: f32) -> Vector {
        Vector {
            x: x2 - x1,
            y: y2 - y1,
        }
    }

    //
    // Add a vector to the current vector
    //
    pub fn add(&mut self, v: &Vector) {
        self.x += v.x;
        self.y += v.y;
    }

    //
    // Remove a vector from the current vector
    //
    pub fn remove(&mut self, v: &Vector) {
        self.x -= v.x;
        self.y -= v.y;
    }

    //
    // Multiply the vector by a floating point number
    //
    //pub fn muliply(&mut self, multiplier: f32) {
    //    self.x *= multiplier;
    //    self.y *= multiplier;
    //}

    //
    // Return a normalized version of the vector
    //
    pub fn normalized(&self) -> Vector {
        let length = self.length();
        Vector {
            x: self.x / length,
            y: self.y / length,
        }
    }

    //
    // Return a multiplied version of the vector
    //
    pub fn multiplied(&self, multiplier: f32) -> Vector {
        Vector {
            x: self.x * multiplier,
            y: self.y * multiplier,
        }
    }

    //
    // Returns the length of the vector
    //
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    //
    // Returns the dot product of two vectors
    //
    //pub fn dot(v1: Vector, v2: Vector) -> f32 {
    //    v1.x * v2.x + v1.y * v2.y
    //}

    //
    // Returns normal of the vector
    // https://stackoverflow.com/a/1252738
    //
    pub fn get_normal(&self) -> Vector {
        return Vector {
            x: -self.y,
            y: self.x,
        };
    }

    //
    // Helper function to get a normalized vector
    //
    // Returns None if the length of the initial vector
    // is inferior or equal to 0
    //
    pub fn get_normalized_vector(x1: f32, y1: f32, x2: f32, y2: f32) -> Option<Vector> {
        let length = Point::get_distance(x1, y1, x2, y2);
        let delta_x = x2 - x1;
        let delta_y = y2 - y1;
        if length > 0.0 {
            let x = delta_x / length;
            let y = delta_y / length;
            Some(Vector { x, y })
        } else {
            None
        }
    }
}
