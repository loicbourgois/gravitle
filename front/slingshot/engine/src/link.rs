use crate::wasm_bindgen;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Link {
    pub a: u32,
    pub b: u32,
}

#[wasm_bindgen]
impl Link {
    pub fn size() -> u32 {
        size_of::<Link>() as u32
    }
}
