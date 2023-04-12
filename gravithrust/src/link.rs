use crate::kind::Kind;
use crate::math::Vector;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Link {
    pub a: usize,
    pub b: usize,
}
#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct LinkJS {
    #[allow(dead_code)]
    pub ak: Kind,
    #[allow(dead_code)]
    pub bk: Kind,
    pub p: Vector,
}
