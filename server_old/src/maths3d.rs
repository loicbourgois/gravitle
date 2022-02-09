pub fn distance_squared_wrap_around(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> f64 {
    let da_squared = distance_squared(x1, y1, z1, x2, y2, z2);
    let db_squared = {
        let b_x1 = (x1 + 0.25).fract();
        let b_y1 = (y1 + 0.25).fract();
        let b_z1 = (z1 + 0.25).fract();
        let b_x2 = (x2 + 0.25).fract();
        let b_y2 = (y2 + 0.25).fract();
        let b_z2 = (z2 + 0.25).fract();
        distance_squared(b_x1, b_y1, b_z1, b_x2, b_y2, b_z2)
    };
    let dc_squared = {
        let c_x1 = (x1 + 0.5).fract();
        let c_y1 = (y1 + 0.5).fract();
        let c_z1 = (z1 + 0.5).fract();
        let c_x2 = (x2 + 0.5).fract();
        let c_y2 = (y2 + 0.5).fract();
        let c_z2 = (z2 + 0.5).fract();
        distance_squared(c_x1, c_y1, c_z1, c_x2, c_y2, c_z2)
    };
    return da_squared.min(db_squared).min(dc_squared);
}
fn distance_squared(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> f64 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    let dz = z2 - z1;
    return dx * dx + dy * dy + dz * dz;
}
pub fn delta_position_wrap_around(
    x1: f64,
    y1: f64,
    z1: f64,
    x2: f64,
    y2: f64,
    z2: f64,
) -> (f64, f64, f64) {
    let da_squared = distance_squared(x1, y1, z1, x2, y2, z2);
    let b_x1 = (x1 + 0.25).fract();
    let b_y1 = (y1 + 0.25).fract();
    let b_z1 = (z1 + 0.25).fract();
    let b_x2 = (x2 + 0.25).fract();
    let b_y2 = (y2 + 0.25).fract();
    let b_z2 = (z2 + 0.25).fract();
    let db_squared = distance_squared(b_x1, b_y1, b_z1, b_x2, b_y2, b_z2);
    let c_x1 = (x1 + 0.5).fract();
    let c_y1 = (y1 + 0.5).fract();
    let c_z1 = (z1 + 0.5).fract();
    let c_x2 = (x2 + 0.5).fract();
    let c_y2 = (y2 + 0.5).fract();
    let c_z2 = (z2 + 0.5).fract();
    let dc_squared = distance_squared(c_x1, c_y1, c_z1, c_x2, c_y2, c_z2);
    if da_squared < db_squared {
        if da_squared < dc_squared {
            return delta(x1, y1, z1, x2, y2, z2);
        } else {
            return delta(c_x1, c_y1, c_z1, c_x2, c_y2, c_z2);
        }
    } else {
        if db_squared < dc_squared {
            return delta(b_x1, b_y1, b_z1, b_x2, b_y2, b_z2);
        }
    }
    return delta(c_x1, c_y1, c_z1, c_x2, c_y2, c_z2);
}
fn delta(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> (f64, f64, f64) {
    return (x1 - x2, y1 - y2, z1 - z2);
}
pub fn normalize(a: (f64, f64, f64)) -> (f64, f64, f64) {
    let d = distance_squared(0.0, 0.0, 0.0, a.0, a.1, a.2).sqrt();
    return (a.0 / d, a.1 / d, a.2 / d);
}
pub fn dot(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> f64 {
    return x1 * x2 + y1 * y2 + z1 * z2;
}
