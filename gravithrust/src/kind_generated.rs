// generated by build.rs
use crate::error;
use serde::Deserialize;
use serde::Serialize;
use wasm_bindgen::prelude::wasm_bindgen;
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Hash, Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Kind {
        Invalid = 0,    Default = 1,    Armor = 2,    Core = 3,    Booster = 4,    Sun = 5,    Light = 6,    Plant = 7,    Metal = 8,    Depot = 9,    Target = 10,    ElectroFieldLauncher = 11,    Cargo = 12,    Plasma = 13,    Field = 14,    Anchor = 15,    SunCore = 16,    ElectroField = 17,    PlasmaElectroField = 18,    PlasmaCargo = 19,    PlasmaElectroFieldCollector = 20,    PlasmaRawDepot = 21,    PlasmaRawCollector = 22,    PlasmaRefineryInput = 23,    PlasmaRefineryOutput = 24,    Static = 25,
}
pub fn kindstr_to_kind(x: &str) -> Kind {
    match x.trim().to_lowercase().as_str() {
            "invalid" => Kind::Invalid,    "default" => Kind::Default,    "armor" => Kind::Armor,    "core" => Kind::Core,    "booster" => Kind::Booster,    "sun" => Kind::Sun,    "light" => Kind::Light,    "plant" => Kind::Plant,    "metal" => Kind::Metal,    "depot" => Kind::Depot,    "target" => Kind::Target,    "electro_field_launcher" => Kind::ElectroFieldLauncher,    "cargo" => Kind::Cargo,    "plasma" => Kind::Plasma,    "field" => Kind::Field,    "anchor" => Kind::Anchor,    "sun_core" => Kind::SunCore,    "electro_field" => Kind::ElectroField,    "plasma_electro_field" => Kind::PlasmaElectroField,    "plasma_cargo" => Kind::PlasmaCargo,    "plasma_electro_field_collector" => Kind::PlasmaElectroFieldCollector,    "plasma_raw_depot" => Kind::PlasmaRawDepot,    "plasma_raw_collector" => Kind::PlasmaRawCollector,    "plasma_refinery_input" => Kind::PlasmaRefineryInput,    "plasma_refinery_output" => Kind::PlasmaRefineryOutput,    "static" => Kind::Static,
        _ => {
            error(&format!("invalid kind: {x}"));
            panic!("invalid kind")
        }
    }
}
impl Kind {
    pub fn capacity(self) -> u32 {
        #[allow(clippy::match_same_arms)]
        match self {
            Kind::PlasmaRefineryInput => 20,Kind::PlasmaElectroField => 1,Kind::PlasmaCargo => 2,Kind::Core => 1,Kind::PlasmaRawDepot => 20,Kind::Booster => 100,Kind::PlasmaElectroFieldCollector => 2,Kind::ElectroFieldLauncher => 2500,Kind::PlasmaRawCollector => 2,Kind::ElectroField => 1,
            _ => 0,
        }
    }
    pub fn soft_capacity(self) -> u32 {
        match self {
            Kind::PlasmaRefineryInput => 20,Kind::PlasmaCargo => 2,Kind::PlasmaRawDepot => 20,
            _ => 0,
        }
    }
    pub fn is_static(self) -> bool {
        matches!(
            self,
            Kind::SunCore|Kind::Depot|Kind::Anchor|Kind::Static
        )
    }
}

