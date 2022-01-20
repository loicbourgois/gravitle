use crate::part::Part;
use crate::Depth;
use crate::PID;
use crate::link::Link;
use crate::ThreadId;
use std::collections::HashSet;

#[derive(Clone)]
pub struct Data {
    pub parts: Vec<Part>,
    pub depths: Vec<Depth>,
    pub step: usize,
    pub links: Vec<Vec<Link>>,
    pub new_pids: Vec<PID>,
    pub parts_to_remove: HashSet<PID>
}
