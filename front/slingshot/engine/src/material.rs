use crate::wasm_bindgen;
use serde::Deserialize;
#[wasm_bindgen]
#[repr(C)] // https://doc.rust-lang.org/nomicon/other-reprs.html#reprc
#[derive(Debug, Deserialize)]
pub struct Material {
    pub density: f32,
}
#[wasm_bindgen]
impl Material {
    pub fn new() -> Material {
        Material { density: 0.0 }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::new()
    }
}
