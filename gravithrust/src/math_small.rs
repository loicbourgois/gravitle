use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}
pub fn rotate(p1: Vector, p2: Vector, angle: f32) -> Vector {
    // Rotates p2 around p1
    // angle should be in [0 ; 1.0]
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
