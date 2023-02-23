use crate::maths::cross;
use crate::maths::dot;
use core::point::Point;

// http://www.sunshine2k.de/coding/java/PointOnLine/PointOnLine.html#step5
pub fn projection(v1: &Point, v2: &Point, p: &Point) -> Point {
    let e1 = Point {
        x: v2.x - v1.x,
        y: v2.y - v1.y,
    };
    let e2 = Point {
        x: p.x - v1.x,
        y: p.y - v1.y,
    };
    let dot_ = dot(&e1, &e2);
    let len2 = e1.x * e1.x + e1.y * e1.y;
    Point {
        x: (v1.x + (dot_ * e1.x) / len2),
        y: (v1.y + (dot_ * e1.y) / len2),
    }
}

#[test]
fn pusher() {
    let d = 1.0;
    let pA = Point { x: 0.0, y: 0.0 };
    let pA_speed = Point { x: 0.5, y: 0.5 };
    let pA_direction = Point { x: 0.0, y: -1.0 };
    let pA_direction_perpendicular = Point {
        x: pA_direction.y,
        y: -pA_direction.x,
    };
    let pa1 = pA - pA_direction_perpendicular;
    let pa2 = pA + pA_direction_perpendicular;
    let pA_new = pA + pA_speed;
    let pp = projection(&pa1, &pa2, &pA_new);
    let cross_ = cross(&pA_direction_perpendicular, &pA_speed);
    println!("pp: {:?}", pp);
    println!("cross_: {:?}", cross_);
}
