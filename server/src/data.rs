use crate::link::Link;
use crate::part::Part;
use crate::Depth;
use crate::Pid;
use std::collections::HashSet;

#[derive(Clone)]
pub struct Data {
    pub parts: Vec<Part>,
    pub depths: Vec<Depth>,
    pub step: usize,
    pub links: Vec<Vec<Link>>,
    pub new_pids: Vec<Pid>,
    pub parts_to_remove: HashSet<Pid>,
}
