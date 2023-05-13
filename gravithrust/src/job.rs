use serde::Deserialize;
use serde::Serialize;
use wasm_bindgen::prelude::wasm_bindgen;
#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    pub tasks: Vec<Task>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub conditions: Vec<Condition>,
    pub action: Action,
}
#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum Condition {
    // PlasmaStorageNotFull,
    // PlasmaStorageFull,
    // CoalStorageNotFull,
    CoalStorageFull,
    CoalStorageEmpty,
    // DeliverCoal,
    // CollectCoal,
    IronOreStorageEmpty,
    IronOreStorageFull,
    // DeliverIronOre,
    // CollectIronOre,
    EnergyStorageEmpty,
    EnergyStorageFull,
    Random1Per1000,
    Random1Per10,
    Random1Per100,
}
#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum Action {
    // CollectPlasmaElectroField,
    // DeliverPlasmaDepot,
    ResetTarget,
    // CollectPlasmaDepot,
    // DeliverPlasmaRefineryIn,
    // LaunchElectroField,
    CollectCoal,
    DeliverCoal,
    DeliverIronOre,
    CollectIronOre,
    CollectEnergy,
    DeliverEnergy,
}
