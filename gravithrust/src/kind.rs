use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u32)]
pub enum Kind {
    Armor = 1,
    Core = 2,
    Booster = 3,
    Sun = 4,
    Light = 5,
    Plant = 6,
}
