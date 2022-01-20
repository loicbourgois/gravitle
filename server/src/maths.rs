//use crate::{Float, Point};

use crate::point::Point;
use crate::Float;

pub fn distance_squared_wrap_around(p1: &Point, p2: &Point) -> Float {
    let da_squared = distance_squared(p1, p2);
    let db_squared = distance_squared(
        &Point {
            x: (p1.x + 0.25).fract(),
            y: (p1.y + 0.25).fract(),
        },
        &Point {
            x: (p2.x + 0.25).fract(),
            y: (p2.y + 0.25).fract(),
        },
    );
    let dc_squared = distance_squared(
        &Point {
            x: (p1.x + 0.5).fract(),
            y: (p1.y + 0.5).fract(),
        },
        &Point {
            x: (p2.x + 0.5).fract(),
            y: (p2.y + 0.5).fract(),
        },
    );
    da_squared.min(db_squared).min(dc_squared)
}
fn distance_squared(p1: &Point, p2: &Point) -> Float {
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    dx * dx + dy * dy
}

pub fn delta_position_wrap_around(a1: &Point, a2: &Point) -> Point {
    let da_squared = distance_squared(a1, a2);
    let b1 = &Point {
        x: (a1.x + 0.25).fract(),
        y: (a1.y + 0.25).fract(),
    };
    let b2 = &Point {
        x: (a2.x + 0.25).fract(),
        y: (a2.y + 0.25).fract(),
    };
    let c1 = &Point {
        x: (a1.x + 0.5).fract(),
        y: (a1.y + 0.5).fract(),
    };
    let c2 = &Point {
        x: (a2.x + 0.5).fract(),
        y: (a2.y + 0.5).fract(),
    };
    let db_squared = distance_squared(b1, b2);
    let dc_squared = distance_squared(c1, c2);

    // match (da_squared < db_squared, da_squared < dc_squared, db_squared < dc_squared) {
    //     (true, true, _) => a1 - a2,
    //     (false, _, true) => b1 - b2,
    //     _ => c1 - c2,
    // }

    if da_squared < db_squared {
        if da_squared < dc_squared {
            a1 - a2
        } else {
            c1 - c2
        }
    } else if db_squared < dc_squared {
        b1 - b2
    } else {
        c1 - c2
    }
}
pub fn normalize(p: &Point) -> Point {
    let d = (p.x * p.x + p.y * p.y).sqrt();
    Point {
        x: p.x / d,
        y: p.y / d,
    }
}
pub fn dot(p1: &Point, p2: &Point) -> Float {
    p1.x * p2.x + p1.y * p2.y
}

pub fn p_coords(p1: &Point, p2: &Point) -> Point {
    rotate(p1, p2, std::f32::consts::PI / 3.0 * 5.0)
}
pub fn rotate(p1: &Point, p2: &Point, angle: Float) -> Point {
    // Rotates p2 around p1
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    let cos_ = angle.cos();
    let sin_ = angle.sin();
    Point {
        x: p1.x + dx * cos_ - dy * sin_,
        y: p1.y + dy * cos_ + dx * sin_,
    }
}
