pub use crate::math_small::Vector;
pub use crate::math_small::rotate;
use std::ops;
impl Default for Vector {
    fn default() -> Self {
        Vector {
            x: 0.0,
            y: 0.0,
        }
    }
}
impl ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl ops::Div<f32> for Vector {
    type Output = Vector;

    fn div(self, other: f32) -> Vector {
        Vector {
            x: self.x / other,
            y: self.y / other,
        }
    }
}
impl ops::Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, other: f32) -> Vector {
        Vector {
            x: self.x * other,
            y: self.y * other,
        }
    }
}
impl ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
#[derive(Clone, Debug)]
pub struct WrapAroundResponse {
    // pub a: Vector,
    // pub b: Vector,
    pub d: Vector,
    pub d_sqrd: f32,
}
pub fn wrap_around(a: Vector, b: Vector) -> WrapAroundResponse {
    let mut dsqrd_min = distance_sqrd(a, b);
    let mut ijwin = [0.0, 0.0];
    let aa = 1.0;
    let ijs = [
        [-aa, -aa],
        [-aa, 0.0],
        [-aa, aa],
        [0.0, -aa],
        [0.0, aa],
        [aa, -aa],
        [aa, 0.0],
        [aa, aa],
    ];
    for ij in ijs {
        let dsqrd = distance_sqrd(
            a,
            Vector {
                x: b.x + ij[0],
                y: b.y + ij[1],
            },
        );
        if dsqrd < dsqrd_min {
            dsqrd_min = dsqrd;
            ijwin = ij;
        }
    }
    let bbb = Vector {
        x: b.x + ijwin[0],
        y: b.y + ijwin[1],
    };
    // let aaa = Vector {
    //     x: a.x + ijwin[0],
    //     y: a.y + ijwin[1],
    // };
    let d = delta(a, bbb);
    WrapAroundResponse {
        // a: aaa,
        // b: bbb,
        d,
        d_sqrd: dsqrd_min,
    }
}
pub fn delta(a: Vector, b: Vector) -> Vector {
    Vector {
        x: b.x - a.x,
        y: b.y - a.y,
    }
}
pub fn distance_sqrd(a: Vector, b: Vector) -> f32 {
    let dp = delta(a, b);
    dp.x * dp.x + dp.y * dp.y
}
pub fn norm_sqrd(v: Vector) -> f32 {
    v.x * v.x + v.y * v.y
}
pub fn norm(v: Vector) -> f32 {
    norm_sqrd(v).sqrt()
}
pub fn normalize(p: Vector, d: f32) -> Vector {
    Vector {
        x: p.x / d,
        y: p.y / d,
    }
}
pub fn normalize_2(p: Vector) -> Vector {
    let d = (p.x * p.x + p.y * p.y).sqrt();
    Vector {
        x: p.x / d,
        y: p.y / d,
    }
}
pub fn dot(a: Vector, b: Vector) -> f32 {
    a.x * b.x + a.y * b.y
}
pub fn cross(p1: Vector, p2: Vector) -> f32 {
    p1.x * p2.y - p1.y * p2.x
}
pub fn radians(x: f32) -> f32 {
    x / 180.0 * std::f32::consts::PI
}
pub fn degrees(x: f32) -> f32 {
    x * (180.0 / std::f32::consts::PI)
}
pub fn angle(p1: Vector, p2: Vector) -> f32 {
    let cross_ = cross(p1, p2);
    let l = norm(p1) * norm(p2);
    let angle = (cross_ / l).asin();
    degrees(angle)
}
