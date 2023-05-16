use crate::job::Job;
use crate::math::wrap_around;
use crate::math::Vector;
use crate::particle::Particle;
pub struct Ship {
    pub p: Vector,
    pub pp: Vector,
    pub v: Vector, // velocity
    pub target: Vector,
    pub td: Vector,                   // target direction
    pub orientation: Vector,          // orientation is where the ship is facing
    pub previous_orientation: Vector, // previous orientation
    pub vt: Vector,
    pub cross: Vector,
    pub on_target: u32,
}
pub struct ShipMore {
    pub pids: Vec<usize>,
    pub ship_control: ShipControl,
    pub anchor_pid: Option<usize>,
    pub target_pid: Option<usize>,
    pub job: Option<Job>,
    pub sid: usize,
    pub orientation_mode: OrientationMode,
}
use serde::Deserialize;
use serde::Serialize;
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum OrientationMode {
    Line,
    Triangle,
}
pub struct ShipControl {
    pub forward: Vec<usize>,
    pub slow: Vec<usize>,
    pub left: Vec<usize>,
    pub right: Vec<usize>,
    pub translate_right: Vec<usize>,
    pub translate_left: Vec<usize>,
}
pub fn ship_position(particles: &[Particle], s: &ShipMore) -> Vector {
    let p0 = &particles[s.pids[0]];
    let mut p = p0.pp;
    for i in 1..s.pids.len() {
        let pid = s.pids[i];
        let p1 = &particles[pid];
        let uu = wrap_around(p0.pp, p1.pp).d;
        p = p + uu / s.pids.len() as f32;
    }
    p
}
pub fn ship_orientation(particles: &[Particle], s: &ShipMore) -> Vector {
    let p0 = &particles[s.pids[0]];
    let p1 = &particles[s.pids[1]];
    match s.orientation_mode {
        OrientationMode::Line => wrap_around(p1.p, p0.p).d,
        OrientationMode::Triangle => {
            let p2 = &particles[s.pids[2]];
            let p12 = p1.p + wrap_around(p1.p, p2.p).d * 0.5;
            wrap_around(p12, p0.p).d
        }
    }
}
