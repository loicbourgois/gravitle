use crate::gravithrust::Gravithrust;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::collections::HashMap;
use std::env;
use std::fs;
pub fn setup_simulation_grid_side(grid_side: u32) -> Gravithrust {
    let envs = env::vars().collect::<HashMap<String, String>>();
    let path = format!(
        "{}/github.com/loicbourgois/gravitle/gravithrust/src/blueprint/plasma_collector.yml",
        envs["HOME"]
    );
    let yaml = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_error) => panic!("Could not read file"),
    };
    let mut gravithrust = Gravithrust::new(
        0.005,       // diameter
        32,          // substep per tick
        0.000000004, // max_rotation_speed
        grid_side,   // grid_side
        0.00001,     // max_speed_at_target
        0.0001,      // forward_max_speed
        30.0,        // forward_max_angle
        35.0,        // slow_down_max_angle
        0.00025,     // slow_down_max_speed_to_target_ratio
        0.00005,     // booster_acceleration
    );
    let mut rng = ChaCha8Rng::seed_from_u64(0);
    for _ in 0..100 {
        gravithrust.add_ship(&yaml, rng.gen::<f32>(), rng.gen::<f32>());
    }
    gravithrust
}
