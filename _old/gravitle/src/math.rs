use crate::Particle;
use crate::Vector;
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
    let ijs = [
        [-1.0, -1.0],
        [-1.0, 0.0],
        [-1.0, 1.0],
        [0.0, -1.0],
        [0.0, 1.0],
        [1.0, -1.0],
        [1.0, 0.0],
        [1.0, 1.0],
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
    WrapAroundResponse {
        a: Vector {
            x: a.x + ijwin[0],
            y: a.y + ijwin[1],
        },
        b: bbb,
        d: delta(a, bbb),
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

pub fn dot(a: Vector, b: Vector) -> f32 {
    a.x * b.x + a.y * b.y
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

// pub fn norm(v: &Vector) -> f32 {
//     (v.x * v.x + v.y * v.y).sqrt()
// }
// pub fn distance(a: &Vector, b: &Vector) -> f32 {
//     let dp = delta(a, b);
//     return (dp.x * dp.x + dp.y * dp.y).sqrt();
// }