use crate::Float;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Point {
    pub x: Float,
    pub y: Float,
}

use std::ops;

// #[derive(Clone, Copy, Debug)]
// pub struct Point {
//     pub x: Float,
//     pub y: Float,
// }

impl ops::Sub<&Point> for &Point {
    type Output = Point;
    fn sub(self, p2: &Point) -> Point {
        Point {
            x: self.x - p2.x,
            y: self.y - p2.y,
        }
    }
}

impl ops::Sub<Point> for Point {
    type Output = Point;
    fn sub(self, p2: Point) -> Point {
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
impl ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, p2: Point) -> Point {
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
impl ops::SubAssign<Point> for Point {
    //type Output = Point;
    fn sub_assign(&mut self, p2: Point) {
        *self = Self {
            x: self.x - p2.x,
            y: self.y - p2.y,
        };
    }
}

// impl ops::DivAssign<Point> for Float {
//     //type Output = Point;
//     fn div_assign(&mut self, f: Float) {
//         *self = Self {
//             x: self.x / f,
//             y: self.y / f,
//         };
//     }
// }

impl ops::Div<Float> for Point {
    type Output = Point;
    fn div(self, f: Float) -> Point {
        Point {
            x: self.x / f,
            y: self.y / f
        }
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
impl ops::Mul<f32> for Point {
    type Output = Point;
    fn mul(self, f: f32) -> Point {
        Point {
            x: self.x * f,
            y: self.y * f,
        }
    }
}

impl Point {
    pub fn new(x: Float, y: Float) -> Point {
        Point { x, y }
    }
}
