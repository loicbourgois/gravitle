use crate::part::Part;
use crate::Depth;
use crate::PartId;
use crate::link::Link;
use crate::ThreadId;

#[derive(Clone)]
pub struct Data {
    pub parts: Vec<Part>,
    pub depths: Vec<Depth>,
    pub step: usize,
    pub links: Vec<Vec<Link>>,
    pub new_pids: Vec<PartId>,
}
