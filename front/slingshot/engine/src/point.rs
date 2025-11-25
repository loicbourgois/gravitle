use crate::wasm_bindgen;
use serde::Serialize;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;
use std::ops::SubAssign;
#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Serialize)]
#[repr(C)] // https://doc.rust-lang.org/nomicon/other-reprs.html#reprc
pub struct Point {
    pub x: f32,
    pub y: f32,
}
#[wasm_bindgen]
impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }
    pub fn normalize(&mut self) -> Point {
        let length = (self.x.powf(2.0) + self.y.powf(2.0)).sqrt();
        if length != 0.0 {
            self.x /= length;
            self.y /= length;
        }
        *self
    }
    pub fn distance(self, b: Point) -> f32 {
        ((self.x - b.x).powf(2.0) + (self.y - b.y).powf(2.0)).sqrt()
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
impl Mul<f32> for Point {
    type Output = Point;
    fn mul(self, scalar: f32) -> Point {
        Point {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}
impl Div<f32> for Point {
    type Output = Point;
    fn div(self, scalar: f32) -> Point {
        Point {
            x: self.x / scalar,
            y: self.y / scalar,
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

// fn equilateral_third_points(a: Point, b: Point) -> (Point, Point) {
//     let vx = b.x - a.x;
//     let vy = b.y - a.y;
//     let angle = std::f32::consts::PI / 3.0; // 60 degrees

//     // Rotate vector by +60 degrees
//     let x1 = vx * angle.cos() - vy * angle.sin();
//     let y1 = vx * angle.sin() + vy * angle.cos();

//     // Rotate vector by -60 degrees
//     let x2 = vx * angle.cos() + vy * angle.sin();
//     let y2 = -vx * angle.sin() + vy * angle.cos();

//     let c1 = Point {
//         x: a.x + x1,
//         y: a.y + y1,
//     };
//     let c2 = Point {
//         x: a.x + x2,
//         y: a.y + y2,
//     };

//     (c1, c2)
// }

pub fn equilateral_third_point(a: Point, b: Point) -> Point {
    let vx = b.x - a.x;
    let vy = b.y - a.y;
    let angle = std::f32::consts::PI / 3.0; // 60 degrees
    // Rotate vector by +60 degrees
    let x1 = vx * angle.cos() - vy * angle.sin();
    let y1 = vx * angle.sin() + vy * angle.cos();
    Point {
        x: a.x + x1,
        y: a.y + y1,
    }
}
