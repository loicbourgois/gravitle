use crate::point::Point;
use crate::wasm_bindgen;

#[wasm_bindgen]
pub struct WrapAroundResult {
    pub a: Point,
    pub b: Point,
    pub d_sqrd: f32,
}

// Collision Response Input
pub struct Cri {
    pub dp: Point,
    pub np: Point,
}

pub fn distance(a: Point, b: Point) -> f32 {
    distance_sqrd(a, b).sqrt()
}

pub fn dot(a: Point, b: Point) -> f32 {
    a.x * b.x + a.y * b.y
}

pub fn collision_response(p1: &Cri, p2: &Cri) -> Point {
    // https://en.wikipedia.org/wiki/Elastic_collision#Two-dimensional_collision_with_two_moving_objects
    let delta_velocity = Point {
        x: p1.dp.x - p2.dp.x,
        y: p1.dp.y - p2.dp.y,
    };
    let delta_position = Point {
        x: p1.np.x - p2.np.x,
        y: p1.np.y - p2.np.y,
    };
    let mass_1 = 1.0;
    let mass_2 = 1.0;
    let mass_factor = 2.0 * mass_2 / (mass_2 + mass_1);
    let dot_vp = dot(delta_velocity, delta_position);
    let distance_ = distance(Point { x: 0.0, y: 0.0 }, delta_position);
    let distance_squared = distance_ * distance_;
    let factor = mass_factor * dot_vp / distance_squared;
    // return acceleration
    Point {
        x: delta_position.x * factor,
        y: delta_position.y * factor,
    }
}

pub fn delta(a: Point, b: Point) -> Point {
    Point {
        x: b.x - a.x,
        y: b.y - a.y,
    }
}

pub fn distance_sqrd(a: Point, b: Point) -> f32 {
    let dp = delta(a, b);
    dp.x * dp.x + dp.y * dp.y
}

/// Facilitates going "around" the map by handling toroidal boundary conditions.
///
/// The map is a square of size 1.0. When a cell goes beyond the boundaries:
/// - 1.1 wraps to 0.1
/// - -0.1 wraps to 0.9
///
/// This function calculates the shortest distance between two points on a unit torus.
/// It returns the original point `a`, the periodic image of point `b` that is
/// closest to `a`, and the squared distance between these two points.
/// Each axis is handled independently, allowing paths to cross one or both boundaries.
///
/// Resources
/// - https://en.wikipedia.org/wiki/Toroidal_graph
#[wasm_bindgen]
pub fn wrap_around_v2(a: Point, b: Point) -> WrapAroundResult {
    // Map size
    let m: f32 = 1.0;
    // Calculate the shortest differences in each dimension considering wrap-around
    let dx = (b.x - a.x + m / 2.0).rem_euclid(m) - m / 2.0;
    let dy = (b.y - a.y + m / 2.0).rem_euclid(m) - m / 2.0;
    let d_sqrd = dx * dx + dy * dy;
    // Return 'a' as-is and 'b' as the nearest periodic image relative to 'a'
    WrapAroundResult {
        a,
        b: Point {
            x: a.x + dx,
            y: a.y + dy,
        },
        d_sqrd,
    }
}


// Known edge case: a = (0.20, 0.20), b = (0.85, 0.85).
// This implementation returns d_sqrd = 0.845 instead of the shortest
// toroidal distance squared, 0.245. Use wrap_around_v2 for correct results.
#[wasm_bindgen]
pub fn wrap_around_v1(a: Point, b: Point) -> WrapAroundResult {
    let o25 = 0.25;
    let o5 = 0.5;
    let m = 1.0;
    let m25 = o25 + m;
    let m5 = o5 + m;
    let a2 = Point {
        x: (a.x + m25) % m,
        y: (a.y + m25) % m,
    };
    let b2 = Point {
        x: (b.x + m25) % m,
        y: (b.y + m25) % m,
    };
    let a3 = Point {
        x: (a.x + m5) % m,
        y: (a.y + m5) % m,
    };
    let b3 = Point {
        x: (b.x + m5) % m,
        y: (b.y + m5) % m,
    };
    let d1 = distance_sqrd(a, b);
    let d2 = distance_sqrd(a2, b2);
    let d3 = distance_sqrd(a3, b3);
    if d1 < d2 {
        if d1 < d3 {
            WrapAroundResult { a, b, d_sqrd: d1 }
        } else {
            WrapAroundResult {
                a: a3,
                b: b3,
                d_sqrd: d3,
            }
        }
    } else if d2 < d3 {
        WrapAroundResult {
            a: a2,
            b: b2,
            d_sqrd: d2,
        }
    } else {
        WrapAroundResult {
            a: a3,
            b: b3,
            d_sqrd: d3,
        }
    }
}
