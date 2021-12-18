use crate::part::Part;
use crate::tests::{DIMENSION, ITERATIONS, PARTS};
use rand;
use rand::Rng;
use std::time::SystemTime;
pub struct Data {
    pub step: usize,
    pub pids: Vec<Vec<Vec<usize>>>,
    pub parts: Vec<Part>,
    pub width: f64,
    pub height: f64,
}
fn init() -> Data {
    let mut d = Data {
        step: 0,
        pids: vec![vec![Vec::new(); DIMENSION]; DIMENSION],
        parts: Vec::new(),
        width: DIMENSION as f64,
        height: DIMENSION as f64,
    };
    let mut rng = rand::thread_rng();
    for _ in 0..PARTS {
        let delta_max = 0.1;
        add_part(&mut AddPartArgs {
            x: d.width * rng.gen::<f64>(),
            y: d.height * rng.gen::<f64>(),
            dx: rng.gen::<f64>() * delta_max * 2.0 - delta_max,
            dy: rng.gen::<f64>() * delta_max * 2.0 - delta_max,
            data: &mut d,
        });
    }
    return d;
}
pub struct AddPartArgs<'a> {
    pub x: f64,
    pub y: f64,
    pub dx: f64,
    pub dy: f64,
    pub data: &'a mut Data,
}
pub fn add_part(x: &mut AddPartArgs) {
    let i: usize = (x.x % x.data.width).floor() as usize;
    let j: usize = (x.y % x.data.height).floor() as usize;
    let pid = x.data.parts.len();
    x.data.parts.push(Part {
        x: x.x,
        y: x.y,
        x_old: x.x - x.dx,
        y_old: x.y - x.dy,
    });
    x.data.pids[i][j].push(pid);
}
#[test]
fn test_init() {
    let start = SystemTime::now();
    for _ in 0..ITERATIONS {
        init();
    }
    println!("vec init: {:?}", start.elapsed().unwrap() / ITERATIONS);
}
#[test]
fn test_clone() {
    let d = init();
    let start = SystemTime::now();
    for _ in 0..ITERATIONS {
        let _ = Data {
            step: d.step,
            parts: d.parts.clone(),
            pids: d.pids.clone(),
            width: d.width,
            height: d.height,
        };
    }
    println!("vec clone: {:?}", start.elapsed().unwrap() / ITERATIONS);
}
