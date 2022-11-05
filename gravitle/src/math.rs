use crate::Vector;
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
