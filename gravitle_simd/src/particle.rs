pub struct Particle {
    pub p: Vector, // position
    pub v: Vector, // velocity
    pub m: f32,    // mass
}

#[derive(Clone, Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug)]
pub struct WrapAroundResponse {
    pub a: Vector,
    pub b: Vector,
    pub d_sqrd: f32,
}

pub fn normalize(v: &Vector) -> Vector {
    let n = norm(v);
    Vector {
        x: v.x / n,
        y: v.y / n,
    }
}

pub fn normalize_inplace(v: &mut Vector) {
    let n = norm(v);
    v.x = v.x / n;
    v.y = v.y / n;
}

pub fn norm(v: &Vector) -> f32 {
    (v.x * v.x + v.y * v.y).sqrt()
}

pub fn norm_sqrd(v: &Vector) -> f32 {
    v.x * v.x + v.y * v.y
}

pub fn collision_response(p1: &Particle, p2: &Particle) -> Vector {
    // https://en.wikipedia.org/wiki/Elastic_collision#Two-dimensional_collision_with_two_moving_objects
    let delta_velocity = Vector {
        x: p1.v.x - p2.v.x,
        y: p1.v.y - p2.v.y,
    };
    let delta_position = Vector {
        x: p1.p.x - p2.p.x, // TODO: should we use np new position ?
        y: p1.p.y - p2.p.y,
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

pub fn dot(a: &Vector, b: &Vector) -> f32 {
    a.x * b.x + a.y * b.y
}

pub fn delta(a: &Vector, b: &Vector) -> Vector {
    return Vector {
        x: b.x - a.x,
        y: b.y - a.y,
    };
}

pub fn distance_sqrd(a: &Vector, b: &Vector) -> f32 {
    let dp = delta(a, b);
    return dp.x * dp.x + dp.y * dp.y;
}

pub fn distance(a: &Vector, b: &Vector) -> f32 {
    let dp = delta(a, b);
    return (dp.x * dp.x + dp.y * dp.y).sqrt();
}

pub fn wrap_around(a: &Vector, b: &Vector) -> WrapAroundResponse {
    let mut dsqrd_min = 1.0;
    let mut bbb = Vector {
        x: b.x,
        y: b.x,
    };
    let ijs = [
        [-1.0, -1.0], [-1.0, 0.0], [-1.0, 1.0],
        [0.0, -1.0], [0.0, 0.0], [0.0, 1.0],
        [1.0, -1.0], [1.0, 0.0], [1.0, 1.0]
    ];
    for ij in ijs {
        let bb = Vector {
            x: (b.x + ij[0]),
            y: (b.y + ij[1]),
        };
        let dsqrd = distance_sqrd(&a, &bb);
        if (dsqrd < dsqrd_min) {
            dsqrd_min = dsqrd;
            bbb=bb;
        }
    }
    return WrapAroundResponse {
        a: Vector {
            x: a.x,
            y: a.x,
        },
        b: bbb,
        d_sqrd: dsqrd_min,
    };
}

pub fn wrap_around_2(a: &Vector, b: &Vector) -> WrapAroundResponse {
    let mut dsqrd_min = 1.0;
    let mut bbb = Vector {
        x: b.x,
        y: b.x,
    };
    let mut ois = [
        [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0],
    ];
    let ijs = [
        [-1.0, -1.0], [-1.0, 0.0], [-1.0, 1.0],
        [0.0, -1.0], [0.0, 0.0], [0.0, 1.0],
        [1.0, -1.0], [1.0, 0.0], [1.0, 1.0]
    ];
    for idx in 0..9 {
        let ij = ijs[idx];
        ois[idx][0] = (b.x + ij[0]);
        ois[idx][1] = (b.x + ij[0]);
        ois[idx][2] = distance_sqrd(&a, &Vector {
            x: (b.x + ij[0]),
            y: (b.y + ij[1]),
        });
    }
    ois.iter().max_by(|a, b| a[2].partial_cmp(&b[2]).unwrap() );
    return WrapAroundResponse {
        a: Vector {
            x: a.x,
            y: a.x,
        },
        b: Vector {
            x: a.x,
            y: a.x,
        },
        d_sqrd: dsqrd_min,
    };
}
