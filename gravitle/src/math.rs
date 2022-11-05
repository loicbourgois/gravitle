use crate::Vector;
use crate::Particle;
#[derive(Clone)]
pub struct WrapAroundResponse {
    pub a: Vector,
    pub b: Vector,
    pub d_sqrd: f32,
}
pub fn wrap_around(a: &Vector, b: &Vector) -> WrapAroundResponse {
    let mut dsqrd_min = distance_sqrd(a, b);
    let mut bbb = Vector { x: b.x, y: b.x };
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
        let bb = Vector {
            x: b.x + ij[0],
            y: b.y + ij[1],
        };
        let dsqrd = distance_sqrd(a, &bb);
        if dsqrd < dsqrd_min {
            dsqrd_min = dsqrd;
            bbb = bb;
        }
    }
    WrapAroundResponse {
        a: Vector { x: a.x, y: a.x },
        b: bbb,
        d_sqrd: dsqrd_min,
    }
}
pub fn delta(a: &Vector, b: &Vector) -> Vector {
    Vector {
        x: b.x - a.x,
        y: b.y - a.y,
    }
}

pub fn distance_sqrd(a: &Vector, b: &Vector) -> f32 {
    let dp = delta(a, b);
    dp.x * dp.x + dp.y * dp.y
}

pub fn collision_response(wa: &WrapAroundResponse, p1: &Particle, p2: &Particle) -> Vector {
    // https://en.wikipedia.org/wiki/Elastic_collision#Two-dimensional_collision_with_two_moving_objects
    let delta_velocity = Vector {
        x: p1.v.x - p2.v.x,
        y: p1.v.y - p2.v.y,
    };
    let delta_position = Vector {
        x: wa.a.x - wa.b.x,
        y: wa.a.y - wa.b.y,
    };
    let mass_factor = 2.0 * p2.m / (p2.m + p1.m);
    let dot_vp = dot(&delta_velocity, &delta_position);
    let n_sqrd = norm_sqrd(&delta_position);
    let factor = mass_factor * dot_vp / n_sqrd;
    let acceleration = Vector {
        x: delta_position.x * factor,
        y: delta_position.y * factor,
    };
    return acceleration;
}
// pub fn norm(v: &Vector) -> f32 {
//     (v.x * v.x + v.y * v.y).sqrt()
// }
pub fn norm_sqrd(v: &Vector) -> f32 {
    v.x * v.x + v.y * v.y
}
pub fn dot(a: &Vector, b: &Vector) -> f32 {
    a.x * b.x + a.y * b.y
}
// pub fn distance(a: &Vector, b: &Vector) -> f32 {
//     let dp = delta(a, b);
//     return (dp.x * dp.x + dp.y * dp.y).sqrt();
// }

// const collision_response = (p1, p2) => {
//   // https://en.wikipedia.org/wiki/Elastic_collision#Two-dimensional_collision_with_two_moving_objects
//   const delta_velocity = {
//     x: p1.dp.x - p2.dp.x,
//     y: p1.dp.y - p2.dp.y,
//   }
//   const delta_position = {
//     x: p1.np.x - p2.np.x,
//     y: p1.np.y - p2.np.y,
//   }
//   let mass_1 = 1.0;
//   let mass_2 = 1.0;
//   let mass_factor = 2.0 * mass_2 / (mass_2 + mass_1);
//   let dot_vp = dot(delta_velocity, delta_position);
//   let distance_ = distance({x:0,y:0}, delta_position);
//   let distance_squared = distance_ * distance_;
//   let factor = mass_factor * dot_vp / distance_squared
//   let acceleration = {
//     x: delta_position.x * factor,
//     y: delta_position.y * factor,
//   };
//   return acceleration
// }
