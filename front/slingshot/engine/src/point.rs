use crate::wasm_bindgen;
#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
#[repr(C)] // https://doc.rust-lang.org/nomicon/other-reprs.html#reprc
pub struct Point {
    pub x: f32,
    pub y: f32,
}
#[wasm_bindgen]
impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }
}
