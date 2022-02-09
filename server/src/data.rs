use crate::link::Link;
use crate::part::Part;
use crate::point::Point;
use crate::Depth;
use crate::DnaId;
use crate::Float;
use crate::Pid;
use std::collections::HashSet;

#[derive(Clone)]
pub struct EntityToAdd {
    pub source_thread_id: usize,
    pub source_dna: DnaId,
    pub total_energy: Float,
    pub position: Point,
}

#[derive(Clone)]
pub struct Data {
    pub parts: Vec<Part>,
    pub depths: Vec<Depth>,
    pub step: usize,
    pub links: Vec<Vec<Link>>,
    pub new_pids: Vec<Pid>,
    pub parts_to_remove: HashSet<Pid>,
    pub entities_to_add: Vec<EntityToAdd>,
}
