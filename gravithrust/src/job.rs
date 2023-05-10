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
    PlasmaStorageNotFull = 1,
    PlasmaStorageFull = 2,
}
#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum Action {
    CollectPlasmaElectroField = 1,
    DeliverPlasmaDepot = 2,
    ResetTarget = 3,
    CollectPlasmaDepot = 4,
    DeliverPlasmaRefineryIn = 5,
    LaunchElectroField = 6,
}
