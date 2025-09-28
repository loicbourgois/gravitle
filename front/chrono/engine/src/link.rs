use crate::log;
use crate::wasm_bindgen;
#[wasm_bindgen]
pub struct Link {
    pub idx: u32,
    pub a: u32,
    pub b: u32,
    pub au: usize,
    pub bu: usize,
}
#[wasm_bindgen]
impl Link {
    pub fn new(idx: u32, a: u32, b: u32) -> Link {
        Link {
            idx,
            a,
            b,
            au: a as usize,
            bu: b as usize,
        }
    }
    pub fn size() -> u32 {
        size_of::<Link>() as u32
    }
}
