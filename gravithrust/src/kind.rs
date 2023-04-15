use crate::log;
use serde::Deserialize;
use serde::Serialize;
use wasm_bindgen::prelude::wasm_bindgen;
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Hash, Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Kind {
    Armor           = 1,
    Core            = 2,
    Booster         = 3,
    Sun             = 4,
    Light           = 5,
    Plant           = 6,
    Metal           = 7,
    Depot           = 8,
    Target          = 9,
    Ray             = 10,
    Cargo           = 11,
    Plasma          = 12,
    Field           = 13,
    Anchor          = 14,
    SunCore         = 15,
    ElectroField    = 16,
    ElectroFieldPlasma = 17,
    PlasmaCargo     = 18,
    PlasmaCollector = 19,
    PlasmaDepot     = 20,
    Static          = 21,
    Default         = 22,
}
impl Kind {
    pub fn capacity(self) -> u32 {
        #[allow(clippy::match_same_arms)]
        match self {
            Kind::Core => 1,
            Kind::Booster => 100,
            Kind::PlasmaCollector => 2,
            Kind::PlasmaCargo => 2,
            Kind::PlasmaDepot => 20,
            Kind::ElectroField => 1,
            Kind::ElectroFieldPlasma => 1,
            Kind::Ray => 2500,
            _ => 0,
        }
    }

    pub fn soft_capacity(self) -> u32 {
        match self {
            Kind::PlasmaCargo => 2,
            Kind::PlasmaDepot => 20,
            _ => 0,
        }
    }

    pub fn is_static(self) -> bool {
        matches!(
            self,
            Kind::SunCore | Kind::Metal | Kind::Depot | Kind::Anchor | Kind::Static
        )
    }
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
        "sun_core" => Kind::SunCore,
        "plasma_cargo" => Kind::PlasmaCargo,
        "plasma_collector" => Kind::PlasmaCollector,
        "plasma_depot" => Kind::PlasmaDepot,
        "static" => Kind::Static,
        _ => {
            log(&format!("invalid kind: {}", x));
            panic!("invalid kind")
        }
    }
}
