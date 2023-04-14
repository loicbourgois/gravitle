use crate::blueprint::load_raw_blueprint;
use crate::blueprint::RawBlueprint;
use crate::gravithrust::Gravithrust;
use crate::job::Action;
use crate::job::Job;
use crate::job::Task;
use std::collections::HashMap;
use std::env;
use std::fs;
#[test]
fn test_job() {
    let envs = env::vars().collect::<HashMap<String, String>>();
    let path = format!(
        "{}/github.com/loicbourgois/gravitle/gravithrust/src/blueprint/blueprint_02.yml",
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
        128,         // grid_side
        0.00001,     // max_speed_at_target
        0.0001,      // forward_max_speed
        30.0,        // forward_max_angle
        35.0,        // slow_down_max_angle
        0.00025,     // slow_down_max_speed_to_target_ratio
        0.00005,     // booster_acceleration
    );
    let sid = gravithrust.add_ship(&yaml, 0.55, 0.5);
    let job = Job {
        tasks: vec![Task {
            conditions: vec![],
            action: Action::CollectElectroFieldPlasma,
        }],
    };
    gravithrust.set_job_internal(sid, job);
    let job_json = r#"{
        "tasks": [
            {
                "conditions": ["StorageNotFull"],
                "action": "CollectElectroFieldPlasma"
            }
        ]
    }"#;
    gravithrust.set_job(sid, job_json);
}
