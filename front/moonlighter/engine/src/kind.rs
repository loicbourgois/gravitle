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
    Wire = 7,
    Switch = 8,
    Led = 9,
    Blower = 10,
    Valve = 11,
    SensorMoonFull = 12,
    SensorMoonEmpty = 13,
    SignalInput = 14,
    SignalOutput = 15,
    SignalWire = 16,
    Trit = 17,
    SignalInverter = 18,
    Press = 19,
    Storage = 20,
    Conveyor = 21,
    Launcher = 22,
    MoonBall = 23,
    Earth = 24,
    Dead = 25,
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
        Kind::Wire => "rgb(209, 117, 71)".to_string(),
        Kind::Switch => "rgb(118, 118, 117)".to_string(),
        Kind::Led => "rgb(51, 113, 57)".to_string(),
        Kind::Blower => "rgb(191, 124, 200)".to_string(),
        Kind::Valve => "rgb(124, 149, 200)".to_string(),

        Kind::SensorMoonFull => "rgb(231, 232, 127)".to_string(),
        Kind::SensorMoonEmpty => "rgb(124, 149, 200)".to_string(),
        Kind::SignalInput => "rgb(124, 149, 200)".to_string(),
        Kind::SignalOutput => "rgb(124, 149, 200)".to_string(),
        Kind::SignalWire => "rgb(168, 172, 118)".to_string(),
        Kind::Trit => "rgb(182, 182, 181)".to_string(),
        Kind::SignalInverter => "rgb(182, 182, 181)".to_string(),

        Kind::Press => "rgb(103, 169, 208)".to_string(),
        Kind::Storage => "rgb(126, 190, 157)".to_string(),
        Kind::Conveyor => "rgb(136, 126, 190)".to_string(),
        Kind::Launcher => "rgb(190, 188, 126)".to_string(),
        Kind::MoonBall => "#eee".to_string(),
        Kind::Earth => "#48f".to_string(),
        Kind::Dead => "#1111".to_string(),
        // _ => "#f0f".to_string(),
    }
}
