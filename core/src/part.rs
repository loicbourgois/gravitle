use crate::point::Point;
use crate::Float;
use serde::Deserialize;
use serde::Serialize;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Part {
    pub d: Float,
    pub m: Float,
    pub p: Point,
    pub pp: Point,
}
