use crate::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub struct WorldConfig {
    pub rdp: f32,
    pub rdv: f32,
    pub zonesize: f32,
    pub gravity: f32,
    pub gravity_2: f32,
    pub crdp: f32,
    pub crdv: f32,
    pub rdv_during_colision: f32,
    pub c2c_gravity: bool,
    pub c2c_colision: bool,
    pub link_strength_dp: f32,
    pub link_strength_dv: f32,
}
