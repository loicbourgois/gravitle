use crate::part::Part;
use crate::Depth;

#[derive(Clone)]
pub struct Data {
    pub parts: Vec<Part>,
    pub depths: Vec<Depth>,
    pub step: usize,
}
