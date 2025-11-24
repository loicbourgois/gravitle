use crate::color::Color;
use crate::wasm_bindgen;
use serde::Deserialize;

#[wasm_bindgen]
#[repr(C)] // https://doc.rust-lang.org/nomicon/other-reprs.html#reprc
#[derive(Debug, Deserialize)]
pub struct Material {
    pub color: Color,
    pub density: f32,
}

#[wasm_bindgen]
impl Material {
    pub fn size() -> u32 {
        size_of::<Material>() as u32
    }
}
