use crate::part::Part;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct DataClient {
    pub step: u32,
    pub width: u32,
    pub height: u32,
    pub i_start: u32,
    pub i_size: u32,
    pub j_start: u32,
    pub j_size: u32,
    pub part_count: u32,
    pub parts: Vec<Part>,
}
