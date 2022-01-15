use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct DataClient {
    pub step: u32,
    pub part_count: u32,
}
