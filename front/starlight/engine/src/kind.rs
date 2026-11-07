use crate::wasm_bindgen;

#[wasm_bindgen]
#[repr(u32)]
#[derive(PartialEq, Copy, Clone)]
pub enum Kind {
    // Ship Part - Movable
    Armor = 0,

    // Ship Part - Movable
    Booster = 1,

    // Ship Part - Movable
    Core = 2,

    // Static
    Asteroid = 4,

    // Static
    Unlighted = 5,

    // Static
    Lighted = 6,

    // Ship Part - Movable
    Headlight = 7,
}
