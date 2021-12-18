use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Part {
    pub x: f64,
    pub y: f64,
    pub x_old: f64,
    pub y_old: f64,
}
