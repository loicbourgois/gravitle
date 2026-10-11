use crate::wasm_bindgen;
#[repr(C)]
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct U32 {
    pub a: u32,
}

#[wasm_bindgen]
impl U32 {
    #[allow(dead_code)]
    pub fn size() -> u32 {
        size_of::<U32>() as u32
    }
}
