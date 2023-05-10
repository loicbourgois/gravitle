use crate::gravithrust::Gravithrust;
use anyhow::Result;
use std::collections::HashMap;
use std::env;
use std::fs;
pub fn setup_simulation() -> Gravithrust {
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
        32,          // grid_side
        0.00001,     // max_speed_at_target
        0.0001,      // forward_max_speed
        30.0,        // forward_max_angle
        35.0,        // slow_down_max_angle
        0.00025,     // slow_down_max_speed_to_target_ratio
        0.00005,     // booster_acceleration
    );
    let _ = gravithrust.add_ship(&yaml, 0.55, 0.5);
    gravithrust
}
pub fn set_job_by_path(g: &mut Gravithrust, path: &str) -> Result<()> {
    let job_json = fs::read_to_string(path)?;
    g.set_job(0, &job_json);
    Ok(())
}
