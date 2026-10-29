use crate::wasm_bindgen;

#[wasm_bindgen]
#[repr(u32)]
#[derive(PartialEq, Copy, Clone)]
pub enum Kind {
    Moon = 0,
    Pipe = 1,
    Tank = 2,
    Filter = 3,
    Pump = 4,
    SolarPanel = 5,
    Battery = 6,
}

#[wasm_bindgen]
pub fn base_color(k: Kind) -> String {
    match k {
        Kind::Moon => "#eee".to_string(), 
        Kind::Pipe => "#b88".to_string(),
        Kind::Tank => "#866".to_string(),
        Kind::Filter => "rgb(136, 164, 255)".to_string(),
        Kind::Pump => "#d88".to_string(),
        Kind::SolarPanel => "rgb(227, 219, 183)".to_string(),
        Kind::Battery => "rgb(204, 194, 147)".to_string(),
        // _ => "#f0f".to_string(),
    }
}
