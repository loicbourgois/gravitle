use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Mul;
use std::ops::Sub;
use std::ops::SubAssign;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}
#[wasm_bindgen]
impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }
    pub fn reset(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
    }
    pub fn normalize(&mut self) -> Point {
        self.normalize_2((self.x.powf(2.0) + self.y.powf(2.0)).sqrt())
    }
    pub fn normalize_2(&mut self, length: f32) -> Point {
        if length != 0.0 {
            self.x /= length;
            self.y /= length;
        }
        *self
    }
    pub fn normalized(self) -> Point {
        let length = (self.x.powf(2.0) + self.y.powf(2.0)).sqrt();
        Point {
            x: self.x / length,
            y: self.y / length,
        }
    }
    pub fn distance(self, b: Point) -> f32 {
        ((self.x - b.x).powf(2.0) + (self.y - b.y).powf(2.0)).sqrt()
    }
    pub fn distance_squared_2(self, b: Point) -> f32 {
        (self.x - b.x).powf(2.0) + (self.y - b.y).powf(2.0)
    }
    pub fn distance_squared(self, x: f32, y: f32) -> f32 {
        (self.x - x).powf(2.0) + (self.y - y).powf(2.0)
    }
    pub fn length(self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }
}
impl Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        self.x += other.x;
        self.y += other.y;
    }
}
impl SubAssign for Point {
    fn sub_assign(&mut self, other: Point) {
        self.x -= other.x;
        self.y -= other.y;
    }
}
impl Mul<f32> for Point {
    type Output = Point;
    fn mul(self, scalar: f32) -> Point {
        Point {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}
