use crate::main_hashmap::{add_part, AddPartArgs, Data};
use crate::tests::{DIMENSION, ITERATIONS, PARTS};
use rand;
use rand::Rng;
use std::collections::HashMap;
use std::time::SystemTime;
fn init() -> Data {
    let mut d = Data {
        step: 0,
        parts: HashMap::new(),
        pids: HashMap::new(),
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
#[test]
fn test_init() {
    let start = SystemTime::now();
    for _ in 0..ITERATIONS {
        init();
    }
    println!("HashMap init: {:?}", start.elapsed().unwrap() / ITERATIONS);
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
    println!("HashMap clone: {:?}", start.elapsed().unwrap() / ITERATIONS);
}
