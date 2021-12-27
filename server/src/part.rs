use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Part {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub x_old: f64,
    pub y_old: f64,
    pub z_old: f64,
    pub colissions: u32,
    pub d: f64,
    pub m: f64,
}
