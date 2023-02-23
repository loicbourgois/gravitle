use rand::Rng;
use std::sync::Arc;
pub type TParticles = Vec<Particle>;
pub type Particles = Vec<Arc<RwLock<TParticles>>>;
pub type TParticleDeltas = Vec<ParticleDelta>;
pub type ParticleDeltas = Vec<Arc<RwLock<TParticleDeltas>>>;
use std::sync::RwLock;

pub struct Particle {
    pub pp: Vector, // previous position
    pub p: Vector,  // position
    pub v: Vector,  // velocity
    pub m: f32,     // mass
    pub thid: usize,
    pub idx: usize,
    pub fidx: usize, // full idx
    pub grid_id: usize,
    pub collisions: u32,
}

pub struct ParticleDelta {
    pub collisions: u32,
    pub dp: Vector, // delta position
    pub thid: usize,
    pub idx: usize,
    pub fidx: usize, // full idx
}

pub fn new_particles(diameter: f32, thread_count: usize, particles_per_thread: usize) -> Particles {
    let mut rng = rand::thread_rng();
    let mut particles: Particles = Vec::new();
    for thid in 0..thread_count {
        let mut t_particles = Vec::new();
        for idx in 0..particles_per_thread {
            let p = Vector {
                x: rng.gen::<f32>(),
                y: rng.gen::<f32>(),
            };
            let v = Vector {
                x: diameter * 0.9 * rng.gen::<f32>() - 0.5 * diameter * 0.9,
                y: diameter * 0.9 * rng.gen::<f32>() - 0.5 * diameter * 0.9,
            };
            let fidx = idx * thread_count + thid;
            t_particles.push(Particle {
                p: p,
                pp: Vector {
                    x: p.x - v.x,
                    y: p.y - v.y,
                },
                v: v,
                m: rng.gen(),
                thid: thid,
                idx: idx,
                grid_id: 0,
                collisions: 0,
                fidx: fidx,
            })
        }
        particles.push(Arc::new(RwLock::new(t_particles)))
    }
    return particles;
}

pub fn new_particle_deltas(thread_count: usize, particles_per_thread: usize) -> ParticleDeltas {
    let mut deltas: ParticleDeltas = Vec::new();
    for thid in 0..thread_count {
        let mut tdeltas = Vec::new();
        for idx in 0..particles_per_thread {
            for thid in 0..thread_count {
                let fidx = idx * thread_count + thid;
                assert!(tdeltas.len() == fidx);
                tdeltas.push(ParticleDelta {
                    dp: Vector { x: 0.0, y: 0.0 },
                    collisions: 0,
                    thid: thid,
                    idx: idx,
                    fidx: fidx,
                })
            }
        }
        deltas.push(Arc::new(RwLock::new(tdeltas)))
    }
    return deltas;
}

#[derive(Clone, Debug, Copy)]
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
    let mut dsqrd_min = distance_sqrd(&a, &b);
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
        let dsqrd = distance_sqrd(&a, &bb);
        if dsqrd < dsqrd_min {
            dsqrd_min = dsqrd;
            bbb = bb;
        }
    }
    return WrapAroundResponse {
        a: Vector { x: a.x, y: a.x },
        b: bbb,
        d_sqrd: dsqrd_min,
    };
}

pub fn wrap_around_3(a: &Vector, b: &Vector) -> WrapAroundResponse {
    let mut dsqrd_min = distance_sqrd(&a, &b);
    if dsqrd_min < 0.25 {
        return WrapAroundResponse {
            a: Vector { x: a.x, y: a.x },
            b: Vector { x: b.x, y: b.x },
            d_sqrd: dsqrd_min,
        };
    } else {
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
                x: (b.x + ij[0]),
                y: (b.y + ij[1]),
            };
            let dsqrd = distance_sqrd(&a, &bb);
            if dsqrd < dsqrd_min {
                dsqrd_min = dsqrd;
                bbb = bb;
            }
        }
        return WrapAroundResponse {
            a: Vector { x: a.x, y: a.x },
            b: bbb,
            d_sqrd: dsqrd_min,
        };
    }
}

pub fn wrap_around_2(a: &Vector, b: &Vector) -> WrapAroundResponse {
    let mut ois = [
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
    ];
    let ijs = [
        [-1.0, -1.0],
        [-1.0, 0.0],
        [-1.0, 1.0],
        [0.0, -1.0],
        [0.0, 0.0],
        [0.0, 1.0],
        [1.0, -1.0],
        [1.0, 0.0],
        [1.0, 1.0],
    ];
    for idx in 0..9 {
        let ij = ijs[idx];
        ois[idx][0] = b.x + ij[0];
        ois[idx][1] = b.x + ij[0];
        ois[idx][2] = distance_sqrd(
            &a,
            &Vector {
                x: b.x + ij[0],
                y: b.y + ij[1],
            },
        );
    }
    let oi = ois
        .iter()
        .max_by(|a, b| a[2].partial_cmp(&b[2]).unwrap())
        .unwrap();
    return WrapAroundResponse {
        a: Vector { x: a.x, y: a.x },
        b: Vector { x: oi[0], y: oi[1] },
        d_sqrd: oi[2],
    };
}

pub fn wrap_around_4(a: &Vector, b: &Vector) -> WrapAroundResponse {
    let mut dsqrd_min = distance_sqrd(&a, &b);
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
        let bbx = b.x + ij[0];
        let bby = b.y + ij[1];
        let dsqrd = distance_sqrd_4(a.x, a.y, bbx, bby);
        if dsqrd < dsqrd_min {
            dsqrd_min = dsqrd;
            bbb = Vector { x: bbx, y: bby };
        }
    }
    return WrapAroundResponse {
        a: Vector { x: a.x, y: a.x },
        b: bbb,
        d_sqrd: dsqrd_min,
    };
}

pub fn distance_sqrd_4(ax: f32, ay: f32, bx: f32, by: f32) -> f32 {
    let dx = ax - bx;
    let dy = ay - by;
    return dx * dx + dy * dy;
}
