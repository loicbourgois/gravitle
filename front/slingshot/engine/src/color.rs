use crate::wasm_bindgen;
use serde::Deserialize;

#[wasm_bindgen]
#[repr(C)]
#[derive(Debug, Deserialize, Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn from_hex(hex: &str) -> Result<Self, String> {
        let hex = hex.trim_start_matches('#');
        let (r, g, b) = match hex.len() {
            3 => {
                let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).map_err(|e| e.to_string())?;
                let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).map_err(|e| e.to_string())?;
                let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).map_err(|e| e.to_string())?;
                (r, g, b)
            }
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).map_err(|e| e.to_string())?;
                let g = u8::from_str_radix(&hex[2..4], 16).map_err(|e| e.to_string())?;
                let b = u8::from_str_radix(&hex[4..6], 16).map_err(|e| e.to_string())?;
                (r, g, b)
            }
            _ => return Err(format!("Invalid hex color: {}", hex)),
        };
        Ok(Color {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
        })
    }
}
