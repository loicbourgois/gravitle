use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Kind {
    Armor = 1,
    Core = 2,
    Booster = 3,
    Sun = 4,
    Light = 5,
    Plant = 6,
    Metal = 7,
    Depot = 8,
    Target = 9,
    Ray = 10,
    Cargo = 11,
}
