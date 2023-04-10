use crate::Vector;

pub struct Ship {
    pub p: Vector,
    pub pp: Vector,
    pub v: Vector, // velocity
    pub target: Vector,
    pub td: Vector, // target direction
    pub orientation: Vector,
    pub vt: Vector,
    pub cross: Vector,
    pub target_pid: usize,
    pub on_target: u32,
    pub anchor_pid: Option<usize>,
}

pub struct ShipMore {
    pub pids: Vec<usize>,
    pub ship_control: ShipControl,
}

pub struct ShipControl {
    pub forward: Vec<usize>,
    pub slow: Vec<usize>,
    pub left: Vec<usize>,
    pub right: Vec<usize>,
    pub translate_right: Vec<usize>,
    pub translate_left: Vec<usize>,
}
