use crate::point::Point;
use crate::Float;

#[derive(Copy, Clone)]
pub struct Part {
    pub d: Float,
    pub m: Float,
    pub p: Point,
    pub pp: Point,
}
