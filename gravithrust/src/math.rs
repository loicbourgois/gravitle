use crate::particle::Particle;
// use crate::Kind;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Delta {
    pub p: Vector,
    pub v: Vector,
    pub direction: Vector,
    pub sid: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct WrapAroundResponse {
    pub a: Vector,
    pub b: Vector,
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
    let aaa = Vector {
        x: a.x + ijwin[0],
        y: a.y + ijwin[1],
    };
    let d = delta(a, bbb);
    WrapAroundResponse {
        a: aaa,
        b: bbb,
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

pub fn collision_response(wa: &WrapAroundResponse, p1: &Particle, p2: &Particle) -> Vector {
    // https://en.wikipedia.org/wiki/Elastic_collision#Two-dimensional_collision_with_two_moving_objects
    let delta_velocity = delta(p1.v, p2.v);
    let delta_position = wa.d;
    let mass_factor = 2.0 * p2.m / (p2.m + p1.m);
    let dot_vp = dot(delta_velocity, delta_position);
    let n_sqrd = norm_sqrd(delta_position);
    let factor = mass_factor * dot_vp / n_sqrd;
    Vector {
        x: delta_position.x * factor,
        y: delta_position.y * factor,
    }
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

pub fn rotate(p1: Vector, p2: Vector, angle: f32) -> Vector {
    // Rotates p2 around p1
    let angle = std::f32::consts::PI * 2.0 * angle;
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    let cos_ = angle.cos();
    let sin_ = angle.sin();
    Vector {
        x: p1.x + dx * cos_ - dy * sin_,
        y: p1.y + dy * cos_ + dx * sin_,
    }
}

pub fn dot(a: Vector, b: Vector) -> f32 {
    a.x * b.x + a.y * b.y
}

pub fn cross(p1: Vector, p2: Vector) -> f32 {
    p1.x * p2.y - p1.y * p2.x
}

pub fn ncross(p1: Vector, p2: Vector) -> f32 {
    cross(normalize_2(p1), normalize_2(p2))
}

// pub fn angle_between_vectors(a: Vector, b: Vector) {
//     const dotProduct = dot(a,b);
//     const magnitudeA = (a.x * a.x + a.y * a.y).sqrt();
//     const magnitudeB = (b.x * b.x + b.y * b.y).sqrt();
//     const angle = (dotProduct / (magnitudeA * magnitudeB)).acos();
//     return angle * ;
//   }

// pub fn angle(p1: Vector, p2: Vector) -> f32 {
//     let dot_ = dot(p1, p2);
//     let l = norm(p1) * norm(p2);
//     let angle = (dot_ / l).acos();
//     return angle * (std::f32::consts::PI * 2.0);
// }

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

// pub fn angle(a: Vector, b: Vector) -> f32 {
//     let dot = a.x*b.x + a.y*b.y;
//     let det = a.x*b.y - a.y*b.x;
//     let angle = atan2(det, dot);
// }

// function getAngleBetweenVectors(a, b) {
//     const aLength = Math.sqrt(a.x * a.x + a.y * a.y);
//     const bLength = Math.sqrt(b.x * b.x + b.y * b.y);
//     const dotProduct = a.x * b.x + a.y * b.y;
//     const angle = Math.acos(dotProduct / (aLength * bLength));
//     return angle;
//   }
