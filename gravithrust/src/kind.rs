use crate::log;
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
    Plasma = 12,
    Field = 13,
    Anchor = 14,
}

pub fn kindstr_to_kind(x: &str) -> Kind {
    match x.trim().to_lowercase().as_str() {
        "armor" => Kind::Armor,
        "core" => Kind::Core,
        "booster" => Kind::Booster,
        "ray" => Kind::Ray,
        "cargo" => Kind::Cargo,
        "sun" => Kind::Sun,
        "target" => Kind::Target,
        "anchor" => Kind::Anchor,
        _ => {
            log(&format!("invalid kind: {}", x));
            panic!("invalid kind")
        }
    }
}
