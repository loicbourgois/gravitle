use crate::{
    delta_position_wrap_around, distance_squared_wrap_around, dot, normalize, wasm_bindgen, Float,
    Part, Point, Server,
};

#[wasm_bindgen]
pub struct CollisionResponse {
    pub force: Point,
    pub delta_position: Point,
    pub linked: bool,
}

pub fn collision(
    p1: &Part,
    p2: &Part,
    server: &Server,
    pid1: &u128,
    pid2: &u128,
) -> CollisionResponse {
    let d_square = distance_squared_wrap_around(&p1.p, &p2.p);
    let dpw = &delta_position_wrap_around(&p1.p, &p2.p);
    let diameter = (p1.d + p2.d) * 0.5;
    let d_link = diameter * 1.2;
    let d_link_squared = d_link * d_link;
    // let do_link = true;
    let linked;
    let link_strength;
    let (pid_min, pid_max) = if pid1 < pid2 {
        (pid1, pid2)
    } else {
        (pid2, pid1)
    };
    match server.links.get(pid_min) {
        Some(a) => match a.get(pid_max) {
            Some(strengh) => {
                linked = true;
                link_strength = strengh;
            }
            None => {
                linked = false;
                link_strength = &0.0;
            }
        },
        None => {
            linked = false;
            link_strength = &0.0;
        }
    }
    let force;
    //let linked;
    let dx_collision;
    let dy_collision;
    if linked
    /*&& d_square < d_link_squared*/
    {
        let norm = normalize(&dpw);
        // let strength = 100.0;
        force = Point {
            x: norm.x * (diameter * diameter - d_square) * link_strength,
            y: norm.y * (diameter * diameter - d_square) * link_strength,
        };
    } else {
        force = Point { x: 0.0, y: 0.0 };
    }
    if d_square < diameter * diameter {
        // https://en.wikipedia.org/wiki/Elastic_collision#Two-dimensional_collision_with_two_moving_objects
        let v1x = p1.p.x - p1.pp.x;
        let v1y = p1.p.y - p1.pp.y;
        let v2x = p2.p.x - p2.pp.x;
        let v2y = p2.p.y - p2.pp.y;
        let dv = &Point {
            x: v1x - v2x,
            y: v1y - v2y,
        };
        let mass_factor = 2.0 * p1.m / (p1.m + p2.m);
        let dot_vp = dot(dv, dpw);
        let acc_x = dpw.x * mass_factor * dot_vp / d_square;
        let acc_y = dpw.y * mass_factor * dot_vp / d_square;
        if linked {
            dx_collision = -acc_x * 0.5;
            dy_collision = -acc_y * 0.5;
        } else {
            dx_collision = -acc_x;
            dy_collision = -acc_y;
        }
    } else {
        dx_collision = 0.0;
        dy_collision = 0.0;
    }
    CollisionResponse {
        force: force,
        delta_position: Point {
            x: dx_collision,
            y: dy_collision,
        },
        linked: linked,
    }
}
