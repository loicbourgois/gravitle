use crate::Float;
use std::ops;
use wasm_bindgen::prelude::wasm_bindgen;
#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: Float,
    pub y: Float,
}
impl ops::Sub<&Point> for &Point {
    type Output = Point;
    fn sub(self, p2: &Point) -> Point {
        Point {
            x: self.x - p2.x,
            y: self.y - p2.y,
        }
    }
}
impl ops::Add<&Point> for &Point {
    type Output = Point;
    fn add(self, p2: &Point) -> Point {
        Point {
            x: self.x + p2.x,
            y: self.y + p2.y,
        }
    }
}
impl ops::AddAssign<&Point> for Point {
    //type Output = Point;
    fn add_assign(&mut self, p2: &Point) {
        *self = Self {
            x: self.x + p2.x,
            y: self.y + p2.y,
        };
    }
}
impl ops::Mul<&Point> for &Point {
    type Output = Point;
    fn mul(self, p2: &Point) -> Point {
        Point {
            x: self.x * p2.x,
            y: self.y * p2.y,
        }
    }
}
#[wasm_bindgen]
impl Point {
    pub fn new(x: Float, y: Float) -> Point {
        Point { x: x, y: y }
    }
}
